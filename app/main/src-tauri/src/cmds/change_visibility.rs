use rqs_lib::Visibility;

use crate::AppState;

#[tauri::command]
pub fn change_visibility(
    message: Visibility,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    info!("change_visibility: {message:?}");

    let mut rqs = state
        .rqs
        .lock()
        .map_err(|_| "RQuickShare state lock is poisoned".to_owned())?;
    rqs.change_visibility(message);
    Ok(())
}
