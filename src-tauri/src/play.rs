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
    let mut next_queued = false;
    let mut prev_pos = Duration::MAX;
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
            let current_pos = guard.current_sink.get_pos();
            if !next_queued {
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
                next_queued = true;
                prev_pos = current_pos;
            } else if next_queued {
                if current_pos < prev_pos {
                    next_queued = false;
                }
                prev_pos = current_pos;
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
    let mut guard = state.state.lock().await;
    guard.current_sink.pause();
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
