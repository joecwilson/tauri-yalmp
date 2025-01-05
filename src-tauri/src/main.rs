// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::sync::Mutex;

use crate::app_state::{AppState, InteriorAppState};
use crate::play::play_current_idx;
use crate::playlist::{load_playlist, set_playlist_idx};
use crate::scan::scan;
use rodio::Sink;
use sqlx::pool::PoolOptions;
use sqlx::SqlitePool;
use tauri::async_runtime::block_on;
use tauri::Manager;
use tokio::fs::OpenOptions;

mod app_state;
mod cpal;
mod play;
mod playlist;
mod scan;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct AlbumSql {
    album_id: i64,
    title: String,
    album_artist: String,
    album_art_path: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct DiscSql {
    disc_id: i64,
    disc_num: i32,
    disc_title: Option<String>,
    disc_art_path: Option<String>,
    album: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct TrackSql {
    track_id: i64,
    track_num: i32,
    track_title: String,
    track_art_path: Option<String>,
    artist: String,
    track_path: String,
    album: i64,
    disc: i64,
}

#[tauri::command]
async fn get_albums(state: tauri::State<'_, AppState>) -> Result<Vec<AlbumSql>, String> {
    let guard = &state.state.lock().unwrap();
    let db = &guard.db;
    let albums = block_on(sqlx::query_as::<_, AlbumSql>("SELECT * from Albums").fetch_all(db))
        .map_err(|e| format!("Failed to get albums {e}"))?;
    return Ok(albums);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DiscTs {
    disc: DiscSql,
    tracks: Vec<TrackSql>,
}

#[tauri::command]
async fn get_discs(
    state: tauri::State<'_, AppState>,
    album_id: i64,
) -> Result<Vec<DiscTs>, String> {
    let guard = &state.state.lock().unwrap();
    let db = &guard.db;
    let discs = block_on(
        sqlx::query_as::<_, DiscSql>("SELECT * from Discs WHERE album = ?1")
            .bind(album_id)
            .fetch_all(db),
    )
    .map_err(|e| format!("Failed to get discs {e}"))?;

    let mut result: Vec<DiscTs> = Vec::new();

    for disc in discs {
        let tracks = block_on(
            sqlx::query_as::<_, TrackSql>("SELECT * from Tracks WHERE disc = ?1")
                .bind(disc.disc_id)
                .fetch_all(db),
        )
        .map_err(|e| format!("Failed to get tracks for disc {e}"))?;
        result.push(DiscTs { disc, tracks });
    }
    return Ok(result);
}

#[tauri::command]
async fn get_tracks(state: tauri::State<'_, AppState>) -> Result<Vec<TrackSql>, String> {
    let guard = &state.state.lock().unwrap();
    let db = &guard.db;
    let tracks = block_on(sqlx::query_as::<_, TrackSql>("SELECT * from Tracks").fetch_all(db))
        .map_err(|e| format!("Failed to get tracks {e}"))?;
    return Ok(tracks);
}

async fn setup_db(app: &tauri::App) -> SqlitePool {
    let mut path = app
        .path_resolver()
        .app_data_dir()
        .expect("could not get data_dir");
    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };
    path.push("db.sqlite");
    let mut options = OpenOptions::new();
    let result = options.create_new(true).write(true).open(&path);
    let mut should_scan = false;
    match result.await {
        Ok(_) => {
            println!("database file created");
            should_scan = true;
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => println!("database file already exists"),
            _ => {
                panic!("error creating database file {}", err);
            }
        },
    }
    let db = PoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();
    let base_folder = Path::new("/home/joseph/Music/.YALMP/albums.json");
    println!("Scanning started");
    if should_scan {
        let _ = scan(base_folder, &db).await;
    }
    println!("scanning complete");
    return db;
}

#[tokio::main]
async fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_albums,
            get_discs,
            get_tracks,
            load_playlist,
            set_playlist_idx,
            play_current_idx,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    let db = setup_db(&app).await;
    let (sink, _queues) = Sink::new_idle();
    let interior_app_state = InteriorAppState {
        db,
        current_playlist: Vec::new(),
        current_playlist_idx: 0,
        current_sink: sink,
        current_sink_output_stream: None,
        current_sink_output_handle: None,
    };
    let state = Mutex::new(interior_app_state);
    app.manage(AppState { state });
    app.run(|_, _| {});
}
