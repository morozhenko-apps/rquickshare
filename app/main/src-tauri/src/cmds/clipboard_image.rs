use std::fs::OpenOptions;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use rqs_lib::{ManagedEphemeralFile, MANAGED_EPHEMERAL_FILE_PREFIX};
use tauri_plugin_clipboard_manager::ClipboardExt;
const MAX_CLIPBOARD_PIXELS: u64 = 64_000_000;

fn next_temp_file() -> Result<(PathBuf, std::fs::File), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();

    for attempt in 0_u8..32 {
        let path = std::env::temp_dir().join(format!(
            "{MANAGED_EPHEMERAL_FILE_PREFIX}{}-{timestamp}-{attempt}.png",
            std::process::id()
        ));

        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(format!("Could not create clipboard image: {error}")),
        }
    }

    Err("Could not allocate a unique clipboard image path".to_owned())
}

#[tauri::command]
pub async fn save_clipboard_image(app: tauri::AppHandle) -> Result<String, String> {
    let clipboard_image = app
        .clipboard()
        .read_image()
        .map_err(|error| format!("Clipboard does not contain a readable image: {error}"))?;

    let width = clipboard_image.width();
    let height = clipboard_image.height();
    let pixels = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or_else(|| "Clipboard image dimensions overflow".to_owned())?;

    if pixels == 0 || pixels > MAX_CLIPBOARD_PIXELS {
        return Err(format!(
            "Clipboard image is too large: {width}x{height} (max {MAX_CLIPBOARD_PIXELS} pixels)"
        ));
    }

    let rgba = clipboard_image.rgba().to_vec();
    let expected_len = usize::try_from(
        pixels
            .checked_mul(4)
            .ok_or_else(|| "Clipboard RGBA size overflow".to_owned())?,
    )
    .map_err(|_| "Clipboard RGBA size does not fit in memory".to_owned())?;

    if rgba.len() != expected_len {
        return Err(format!(
            "Clipboard image has invalid RGBA length: {} != {expected_len}",
            rgba.len()
        ));
    }

    let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, rgba)
        .ok_or_else(|| "Clipboard image dimensions do not match its pixel buffer".to_owned())?;
    let image = DynamicImage::ImageRgba8(buffer);

    let (path, mut file) = next_temp_file()?;
    if let Err(error) = image.write_to(&mut file, ImageFormat::Png) {
        let _ = std::fs::remove_file(&path);
        return Err(format!("Could not encode clipboard image: {error}"));
    }

    Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn remove_ephemeral_file(path: String) -> Result<(), String> {
    let managed = ManagedEphemeralFile::try_from_path(path).map_err(|error| error.to_string())?;
    managed.remove().map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn remove_ephemeral_file_uses_core_managed_path_guard() {
        let (path, file) = next_temp_file().unwrap();
        drop(file);

        remove_ephemeral_file(path.to_string_lossy().into_owned())
            .await
            .unwrap();
        assert!(!path.exists());

        remove_ephemeral_file(path.to_string_lossy().into_owned())
            .await
            .unwrap();
        assert!(remove_ephemeral_file("/etc/passwd".to_owned()).await.is_err());
        assert!(
            remove_ephemeral_file(
                std::env::temp_dir()
                    .join("other-app.png")
                    .to_string_lossy()
                    .into_owned()
            )
            .await
            .is_err()
        );
    }
}
