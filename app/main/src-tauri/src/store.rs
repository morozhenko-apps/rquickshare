use std::{path::PathBuf, sync::Arc, time::Duration};

use rqs_lib::Visibility;
use tauri::{AppHandle, Emitter, Wry};
use tauri_plugin_store::{Store, StoreExt};

fn get_store(app_handle: &AppHandle) -> Result<Arc<Store<Wry>>, anyhow::Error> {
    app_handle
        .store_builder(".settings.json")
        .auto_save(Duration::from_millis(100))
        .build()
        .map_err(|error| anyhow::anyhow!("unable to open settings store: {error}"))
}

pub fn init_default(app_handle: &AppHandle) {
    let store = match get_store(app_handle) {
        Ok(store) => store,
        Err(error) => {
            error!("init_default: {error}");
            return;
        }
    };

    if !store.has("autostart") {
        store.set("autostart", false);
    }

    if !store.has("realclose") {
        store.set("realclose", false);
    }

    if !store.has("visibility") {
        store.set("visibility", Visibility::Visible as u8);
    }

    if !store.has("startminimized") {
        store.set("startminimized", false);
    }
}

pub fn get_realclose(app_handle: &AppHandle) -> bool {
    get_store(app_handle)
        .ok()
        .and_then(|store| store.get("realclose"))
        .and_then(|json| json.as_bool())
        .unwrap_or_default()
}

pub fn get_port(app_handle: &AppHandle) -> Option<u32> {
    get_store(app_handle)
        .ok()
        .and_then(|store| store.get("port"))
        .and_then(|json| json.as_u64())
        .and_then(|value| u32::try_from(value).ok())
}

pub fn get_visibility(app_handle: &AppHandle) -> Visibility {
    get_store(app_handle)
        .ok()
        .and_then(|store| store.get("visibility"))
        .and_then(|json| json.as_u64())
        .map(Visibility::from_raw_value)
        .unwrap_or(Visibility::Visible)
}

pub fn set_visibility(app_handle: &AppHandle, visibility: Visibility) -> Result<(), anyhow::Error> {
    let store = get_store(app_handle)?;

    store.set("visibility", visibility as u8);
    app_handle.emit("visibility_updated", ())?;

    Ok(())
}

pub fn get_download_path(app_handle: &AppHandle) -> Option<PathBuf> {
    get_store(app_handle)
        .ok()
        .and_then(|store| store.get("download_path"))
        .and_then(|json| json.as_str().map(PathBuf::from))
}

pub fn get_logging_level(app_handle: &AppHandle) -> Option<String> {
    get_store(app_handle)
        .ok()
        .and_then(|store| store.get("debug_level"))
        .and_then(|json| json.as_str().map(String::from))
}

pub fn get_startminimized(app_handle: &AppHandle) -> bool {
    get_store(app_handle)
        .ok()
        .and_then(|store| store.get("startminimized"))
        .and_then(|json| json.as_bool())
        .unwrap_or_default()
}
