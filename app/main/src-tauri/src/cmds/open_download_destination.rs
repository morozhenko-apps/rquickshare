use std::path::{Component, Path, PathBuf};
#[cfg(target_os = "linux")]
use std::time::Duration;

#[cfg(target_os = "linux")]
use dbus::blocking::Connection;
use tauri::{AppHandle, Manager};
#[cfg(target_os = "linux")]
use url::Url;

use crate::store::get_download_path;

fn validate_download_destination(path: &Path) -> Result<(), String> {
    if !path.is_dir() {
        return Err(format!(
            "download destination is not an existing directory: {}",
            path.display()
        ));
    }

    Ok(())
}

fn validate_received_relative_path(value: &str) -> Result<Vec<&str>, String> {
    if value.is_empty() || value.len() > 4096 {
        return Err("received relative path is empty or too long".to_owned());
    }

    let segments = value.split('/').collect::<Vec<_>>();
    if segments.len() > 64 {
        return Err("received relative path is too deep".to_owned());
    }

    for segment in &segments {
        if segment.is_empty()
            || *segment == "."
            || *segment == ".."
            || segment.len() > 255
            || segment
                .chars()
                .any(|character| character == '\\' || character == '\0' || character.is_control())
        {
            return Err("received relative path contains an unsafe component".to_owned());
        }

        let mut components = Path::new(segment).components();
        if !matches!(
            (components.next(), components.next()),
            (Some(Component::Normal(_)), None)
        ) {
            return Err("received relative path contains an unsafe component".to_owned());
        }
    }

    Ok(segments)
}

fn resolve_download_destination(app: &AppHandle) -> Result<PathBuf, String> {
    let destination = match get_download_path(app) {
        Some(path) => path,
        None => app
            .path()
            .download_dir()
            .map_err(|error| format!("unable to resolve the system download directory: {error}"))?,
    };

    validate_download_destination(&destination)?;
    destination
        .canonicalize()
        .map_err(|error| format!("unable to resolve the download destination: {error}"))
}

fn resolve_received_item(
    destination: &Path,
    relative_path: &str,
) -> Result<Option<PathBuf>, String> {
    let segments = validate_received_relative_path(relative_path)?;
    let candidate = segments
        .iter()
        .fold(destination.to_path_buf(), |path, segment| path.join(segment));

    if !candidate.exists() {
        return Ok(None);
    }

    let canonical_candidate = candidate
        .canonicalize()
        .map_err(|error| format!("unable to resolve received item: {error}"))?;

    if !canonical_candidate.starts_with(destination) {
        return Err("received item escaped the download destination".to_owned());
    }

    Ok(Some(canonical_candidate))
}

#[cfg(target_os = "linux")]
fn show_item_in_file_manager(path: &Path) -> Result<(), String> {
    let uri = Url::from_file_path(path)
        .map_err(|_| format!("unable to convert path to file URI: {}", path.display()))?;

    let connection = Connection::new_session()
        .map_err(|error| format!("unable to connect to the session D-Bus: {error}"))?;
    let proxy = connection.with_proxy(
        "org.freedesktop.FileManager1",
        "/org/freedesktop/FileManager1",
        Duration::from_secs(3),
    );

    let _: () = proxy
        .method_call(
            "org.freedesktop.FileManager1",
            "ShowItems",
            (vec![uri.to_string()], String::new()),
        )
        .map_err(|error| format!("FileManager1.ShowItems failed: {error}"))?;

    Ok(())
}

#[tauri::command]
pub fn open_download_destination(app: AppHandle) -> Result<(), String> {
    let destination = resolve_download_destination(&app)?;
    info!("open_download_destination: {:?}", destination);

    open::that(&destination)
        .map_err(|error| format!("unable to open the download destination: {error}"))
}

#[tauri::command]
pub fn reveal_download_item(app: AppHandle, file_name: String) -> Result<(), String> {
    let destination = resolve_download_destination(&app)?;
    let item = resolve_received_item(&destination, &file_name)?;

    #[cfg(target_os = "linux")]
    if let Some(item) = item.as_ref() {
        info!("reveal_download_item: {:?}", item);
        if let Err(error) = show_item_in_file_manager(item) {
            warn!("{error}; falling back to opening the download directory");
        } else {
            return Ok(());
        }
    }

    #[cfg(not(target_os = "linux"))]
    let _ = item;

    open::that(&destination)
        .map_err(|error| format!("unable to open the download destination: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destination_validation_accepts_directories_and_rejects_missing_paths() {
        let temp = std::env::temp_dir();
        assert!(validate_download_destination(&temp).is_ok());

        let missing = temp.join(format!(
            "rquickshare-missing-download-destination-{}",
            std::process::id()
        ));
        assert!(validate_download_destination(&missing).is_err());
    }

    #[test]
    fn received_relative_path_accepts_nested_items_and_rejects_traversal() {
        for value in [
            "",
            ".",
            "..",
            "../secret",
            "/tmp/secret",
            "folder//file.png",
            "folder/../file.png",
            "folder\\file.png",
        ] {
            assert!(
                validate_received_relative_path(value).is_err(),
                "unsafe path unexpectedly accepted: {value}"
            );
        }

        assert!(validate_received_relative_path("photo 01.png").is_ok());
        assert!(validate_received_relative_path("Trip/photos/данные.txt").is_ok());
    }

    #[test]
    fn received_item_must_exist_under_destination() {
        let root = std::env::temp_dir().join(format!(
            "rquickshare-reveal-test-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("thread")
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let destination = root.canonicalize().unwrap();

        let nested = destination.join("Trip").join("photos");
        std::fs::create_dir_all(&nested).unwrap();
        let file = nested.join("photo.png");
        std::fs::write(&file, b"test").unwrap();

        assert_eq!(
            resolve_received_item(&destination, "Trip/photos/photo.png")
                .unwrap()
                .unwrap(),
            file.canonicalize().unwrap()
        );
        assert!(resolve_received_item(&destination, "Trip/photos/missing.png")
            .unwrap()
            .is_none());
        assert!(resolve_received_item(&destination, "../photo.png").is_err());

        std::fs::remove_dir_all(&root).unwrap();
    }
}
