use crate::app_state::AppState;
use std::string::String;
use std::vec::Vec;

#[tauri::command]
pub async fn load_playlist(
    state: tauri::State<'_, AppState>,
    new_playlist: Vec<String>,
) -> Result<(), String> {
    let guard = &mut state.state.lock().await;
    guard.current_playlist = new_playlist;
    guard.current_playlist_idx = 0;
    return Ok(());
}

#[tauri::command]
pub async fn set_playlist_idx(
    state: tauri::State<'_, AppState>,
    new_idx: usize,
) -> Result<(), String> {
    let guard = &mut state.state.lock().await;
    if guard.current_playlist.len() < new_idx {
        guard.current_playlist_idx = new_idx
    }
    return Ok(());
}
