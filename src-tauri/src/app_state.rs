use sqlx::SqlitePool;
use std::sync::Mutex;

pub struct InteriorAppState {
    pub db: SqlitePool,
    pub current_playlist: Vec<String>,
    pub current_playlist_idx: usize,
}
pub struct AppState {
    pub state: Mutex<InteriorAppState>,
}
