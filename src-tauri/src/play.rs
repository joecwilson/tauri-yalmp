use crate::app_state::AppState;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use tokio::time;
use tokio::time::Duration;

#[tauri::command]
pub async fn play_current_idx(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.state.lock().await;

    if !guard.stopped {
        guard.current_sink.play();
        return Ok(());
    }
    guard.stopped = false;
    let mut interval = time::interval(Duration::from_millis(10));

    let play_counter = guard.requested_playing_counter + 1;
    guard.requested_playing_counter = play_counter;
    drop(guard);

    let mut current_song = get_current_song(&state).await.unwrap();
    let mut open_file = File::open(current_song).map_err(|_| format!("Failed to open file"))?;
    let mut file = BufReader::new(open_file);
    // // Decode that sound file into a source
    let mut source = Decoder::new(file).map_err(|_| format!("Failed to decode file"))?;
    guard = state.state.lock().await;
    guard.current_sink.append(source);
    guard.current_sink.play();
    drop(guard);
    loop {
        let mut guard = state.state.lock().await;
        if !guard.requested_playing_counter == play_counter {
            drop(guard);
            break;
        }
        if guard.current_playing_counter == 0 {
            guard.current_playing_counter = play_counter;
        }
        if guard.current_playing_counter == play_counter {
            if guard.current_sink.len() <= 1 {
                println!("Current length = {}", guard.current_sink.len());
                if guard.current_playlist_idx + 1 < guard.current_playlist.len() {
                    guard.current_playlist_idx += 1;
                    current_song = guard
                        .current_playlist
                        .get(guard.current_playlist_idx)
                        .expect("Should have valid index")
                        .to_string();

                    open_file =
                        File::open(&current_song).map_err(|_| format!("Failed to open file"))?;
                    file = BufReader::new(open_file);
                    source = Decoder::new(file).map_err(|_| format!("Failed to decode file"))?;
                    guard.current_sink.append(source);
                }
            } 
        }
        drop(guard);
        interval.tick().await;
    }
    let mut guard = state.state.lock().await;
    guard.current_sink.clear();
    guard.current_playing_counter = guard.requested_playing_counter;

    return Ok(());
}

#[tauri::command]
pub async fn pause_song(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let guard = state.state.lock().await;
    guard.current_sink.pause();
    return Ok(());
}

#[tauri::command]
pub async fn get_current_location(state: tauri::State<'_, AppState>) -> Result<u128, String> {
    let guard = state.state.lock().await;
    let time = guard.current_sink.get_pos();
    return Ok(time.as_millis().try_into().unwrap());
}

#[tauri::command]
pub async fn seek_song(state: tauri::State<'_, AppState>, milliseconds: u64) -> Result<(), String> {
    let guard = state.state.lock().await;
    guard
        .current_sink
        .try_seek(Duration::from_millis(milliseconds))
        .map_err(|_| "Failed to seek to requested position")?;
    return Ok(());
}

async fn get_current_song(state: &tauri::State<'_, AppState>) -> Result<String, String> {
    let guard = &state.state.lock().await;
    let current_song = guard.current_playlist.get(guard.current_playlist_idx);
    return match current_song {
        None => Err(format!("Invalid index for playlist")),
        Some(p) => Ok(p.clone()),
    };
}
