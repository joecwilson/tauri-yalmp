use sqlx::SqlitePool;
use std::sync::Mutex;

pub struct InteriorAppState {
    pub db: SqlitePool,
    pub current_playlist: Vec<String>,
    pub current_playlist_idx: i32,
}
pub struct AppState {
    pub state: Mutex<InteriorAppState>,
}
