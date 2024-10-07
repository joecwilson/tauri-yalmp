use std::string::String;
use std::vec::Vec;

use crate::app_state::AppState;

#[tauri::command]
pub fn load_playlist(state: tauri::State<'_, AppState>, new_playlist: Vec<String>) {
    let guard = &mut state.state.lock().unwrap();
    guard.current_playlist = new_playlist;
    guard.current_playlist_idx = 0;
}
