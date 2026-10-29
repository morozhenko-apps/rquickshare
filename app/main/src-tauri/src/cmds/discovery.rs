use crate::AppState;

#[tauri::command]
pub async fn start_discovery(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("start_discovery");

    // AppState uses std::sync::Mutex for synchronous Core commands.
    // Run the asynchronous join inside the existing multithreaded Tokio
    // blocking bridge, so no MutexGuard crosses the outer command await.
    tokio::task::block_in_place(|| {
        let mut rqs = state
            .rqs
            .lock()
            .map_err(|_| "RQuickShare state lock is poisoned".to_owned())?;
        tauri::async_runtime::block_on(rqs.discovery(state.dch_sender.clone()))
            .map_err(|error| format!("unable to start discovery: {error}"))
    })
}

#[tauri::command]
pub async fn stop_discovery(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("stop_discovery");

    tokio::task::block_in_place(|| {
        let mut rqs = state
            .rqs
            .lock()
            .map_err(|_| "RQuickShare state lock is poisoned".to_owned())?;
        tauri::async_runtime::block_on(rqs.stop_discovery_and_wait())
            .map_err(|error| format!("unable to stop discovery: {error}"))
    })
}
