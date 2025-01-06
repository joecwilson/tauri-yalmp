use std::string::String;
use std::vec::Vec;

use crate::app_state::AppState;

#[tauri::command]
pub fn load_playlist(state: tauri::State<'_, AppState>, new_playlist: Vec<String>) {
    println!("Load playlist called");
    let guard = &mut state.state.lock().unwrap();
    println!("new_playlist was {new_playlist:#?}");
    guard.current_playlist = new_playlist;
    guard.current_playlist_idx = 0;
}

#[tauri::command]
pub fn set_playlist_idx(state: tauri::State<'_, AppState>, new_idx: usize) {
    let guard = &mut state.state.lock().unwrap();
    if guard.current_playlist.len() < new_idx {
        guard.current_playlist_idx = new_idx
    }
}
