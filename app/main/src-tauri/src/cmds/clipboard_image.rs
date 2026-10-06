use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use tauri_plugin_clipboard_manager::ClipboardExt;

const CLIPBOARD_FILE_PREFIX: &str = "rquickshare-clipboard-";
const MAX_CLIPBOARD_PIXELS: u64 = 64_000_000;

fn managed_temp_path(path: &Path) -> bool {
    path.parent() == Some(std::env::temp_dir().as_path())
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(CLIPBOARD_FILE_PREFIX) && name.ends_with(".png"))
}

fn next_temp_file() -> Result<(PathBuf, std::fs::File), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();

    for attempt in 0_u8..32 {
        let path = std::env::temp_dir().join(format!(
            "{CLIPBOARD_FILE_PREFIX}{}-{timestamp}-{attempt}.png",
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
    let path = PathBuf::from(path);
    if !managed_temp_path(&path) {
        return Err("Refusing to remove a path outside managed clipboard temp files".to_owned());
    }

    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("Could not remove clipboard image: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn managed_temp_path_accepts_only_our_png_prefix() {
        let good = std::env::temp_dir().join("rquickshare-clipboard-1-2-0.png");
        assert!(managed_temp_path(&good));

        assert!(!managed_temp_path(
            &std::env::temp_dir().join("other-app.png")
        ));
        assert!(!managed_temp_path(Path::new("/etc/passwd")));
    }
}
