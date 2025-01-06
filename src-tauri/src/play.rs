use crate::app_state::{AppState, SendStream};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

#[tauri::command]
pub fn play_current_idx(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let current_song = get_current_song(&state).unwrap();
    println!("current song is {current_song}");
    let (stream, stream_handle) =
        OutputStream::try_default().map_err(|_| format!("Failed to open stream"))?;
    let open_file = File::open(current_song).map_err(|_| format!("Failed to open file"))?;
    let file = BufReader::new(open_file);
    // Decode that sound file into a source
    let source = Decoder::new(file).map_err(|_| format!("Failed to decode file"))?;
    let mut guard = state.state.lock().unwrap();
    let new_sink =
        Sink::try_new(&stream_handle).map_err(|_| format!("Failed to create new audio sink"))?;
    new_sink.append(source);
    let send_stream = SendStream(stream);
    guard.current_sink = new_sink;
    guard.current_sink.set_volume(0.1);
    guard.current_sink_output_handle = Some(stream_handle);
    guard.current_sink_output_stream = Some(send_stream);
    // drop(guard)
    guard.current_sink.play();
    // todo!()
    guard.current_sink.sleep_until_end();
    drop(guard);
    play_next_song_at_end(&state);

    // Need to spawn a new thread that sleeps till the end of the stream and plays a song
    return Ok(());
}

fn play_next_song_at_end(_state: &tauri::State<'_, AppState>) {
    // let guard = state.state.lock().unwrap();
    // Given a sink
}

fn get_current_song(state: &tauri::State<'_, AppState>) -> Result<String, String> {
    let guard = &state.state.lock().unwrap();
    let current_song = guard.current_playlist.get(guard.current_playlist_idx);
    match current_song {
        None => return Err(format!("Invalid index for playlist")),
        Some(p) => return Ok(p.clone()),
    };
}
