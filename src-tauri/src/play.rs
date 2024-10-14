use crate::app_state::{AppState, SendStream};
use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fmt::format;
use std::fs::File;
use std::io::BufReader;

#[tauri::command]
pub fn play_current_idx(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let current_song = get_current_song(&state)?;
    let (stream, stream_handle) =
        OutputStream::try_default().map_err(|_| format!("Failed to open stream"))?;
    let open_file = File::open(current_song).map_err(|_| format!("Failed to open file"))?;
    let file = BufReader::new(open_file);
    // Decode that sound file into a source
    let source = Decoder::new(file).map_err(|_| format!("Failed to decode file"))?;
    let mut guard = state.state.lock().unwrap();
    let new_sink = Sink::try_new(&stream_handle).map_err(|_| format!("Failed to create new audio sink"))?;
    new_sink.append(source);
    let send_stream = SendStream(stream);
    guard.current_sink = new_sink;
    guard.current_sink_output_handle = Some(stream_handle);
    guard.current_sink_output_stream = Some(send_stream);
    return Ok(());
}

fn get_current_song(state: &tauri::State<'_, AppState>) -> Result<String, String>{
    let guard = &state.state.lock().unwrap();
    let current_song = guard.current_playlist.get(guard.current_playlist_idx);
    match current_song {
        None => return Err(format!("Invalid index for playlist")),
        Some(p) => return Ok(p.clone())
    };
}
