use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager};

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

#[tauri::command]
pub fn open_download_destination(app: AppHandle) -> Result<(), String> {
    let destination = resolve_download_destination(&app)?;
    info!("open_download_destination: {:?}", destination);

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
}
