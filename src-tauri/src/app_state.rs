use rodio::{OutputStream, OutputStreamHandle, Sink};
use rusqlite::Connection;
use tokio::sync::Mutex;

pub struct SendStream(pub OutputStream);
unsafe impl Send for SendStream {}

pub struct InteriorAppState {
    pub db: Connection,
    pub current_playlist: Vec<String>,
    pub current_playlist_idx: usize,
    pub current_sink: Sink,
    pub current_sink_output_stream: Option<SendStream>,
    pub current_sink_output_handle: Option<OutputStreamHandle>,
    pub current_playing_counter: usize,
    pub requested_playing_counter: usize,
    pub stopped: bool,
}
pub struct AppState {
    pub state: Mutex<InteriorAppState>,
}
