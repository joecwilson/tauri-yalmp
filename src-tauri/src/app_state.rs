use rodio::{OutputStream, OutputStreamHandle, Sink};
use sqlx::SqlitePool;
use std::sync::Mutex;


pub struct SendStream(pub OutputStream);
unsafe impl Send for SendStream{}

pub struct InteriorAppState {
    pub db: SqlitePool,
    pub current_playlist: Vec<String>,
    pub current_playlist_idx: usize,
    pub current_sink: Sink,
    pub current_sink_output_stream: Option<SendStream>,
    pub current_sink_output_handle: Option<OutputStreamHandle>,
}
pub struct AppState {
    pub state: Mutex<InteriorAppState>,
}
