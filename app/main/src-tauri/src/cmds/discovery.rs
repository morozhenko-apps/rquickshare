use crate::AppState;

#[tauri::command]
pub async fn start_discovery(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("start_discovery");

    let mut rqs = state
        .rqs
        .lock()
        .map_err(|_| "RQuickShare state lock is poisoned".to_owned())?;
    rqs.discovery(state.dch_sender.clone())
        .map_err(|error| format!("unable to start discovery: {error}"))
}

#[tauri::command]
pub fn stop_discovery(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("stop_discovery");

    let mut rqs = state
        .rqs
        .lock()
        .map_err(|_| "RQuickShare state lock is poisoned".to_owned())?;
    rqs.stop_discovery();
    Ok(())
}
