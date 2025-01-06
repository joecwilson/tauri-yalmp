// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::sync::Mutex;

use crate::app_state::{AppState, InteriorAppState};
use crate::play::play_current_idx;
use crate::playlist::{load_playlist, set_playlist_idx};
use crate::scan::scan;
use dirs;
use rodio::Sink;
use rusqlite::{Connection, Result};
use sqlx::pool::PoolOptions;
use sqlx::SqlitePool;
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
    let mut statement = db.prepare_cached("SELECT * from Albums").unwrap();
    let album_iterator = statement
        .query_map([], |row| {
            Ok(AlbumSql {
                album_id: row.get(0)?,
                title: row.get(1)?,
                album_artist: row.get(2)?,
                album_art_path: row.get(3)?,
            })
        })
        .unwrap();
    let mut all_albums = Vec::new();
    for album in album_iterator {
        all_albums.push(album.unwrap());
    }
    return Ok(all_albums);
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

    let mut disc_statement = db
        .prepare_cached("SELECT * from Discs WHERE album = ?1")
        .unwrap();

    let disc_iterator = disc_statement
        .query_map([album_id], |row| {
            Ok(DiscSql {
                disc_id: row.get(0)?,
                disc_num: row.get(1)?,
                disc_title: row.get(2)?,
                disc_art_path: row.get(3)?,
                album: row.get(5)?,
            })
        })
        .unwrap();
    let mut result: Vec<DiscTs> = Vec::new();

    let mut track_statement = db
        .prepare_cached("SELECT * from Tracks WHERE disc = ?1")
        .unwrap();

    for disc in disc_iterator {
        let unwrapped_disc = disc.unwrap();
        let track_iterator = track_statement
            .query_map([unwrapped_disc.disc_id], |row| {
                Ok(TrackSql {
                    track_id: row.get(0)?,
                    track_num: row.get(1)?,
                    track_title: row.get(2)?,
                    track_art_path: row.get(3)?,
                    artist: row.get(4)?,
                    track_path: row.get(5)?,
                    album: row.get(7)?,
                    disc: row.get(8)?,
                })
            })
            .unwrap();
        let mut track_list: Vec<TrackSql> = Vec::new();
        for track in track_iterator {
            track_list.push(track.unwrap());
        }
        result.push(DiscTs {
            disc: (unwrapped_disc),
            tracks: (track_list),
        })

        // let tracks = block_on(
        //     sqlx::query_as::<_, TrackSql>("SELECT * from Tracks WHERE disc = ?1")
        //         .bind(disc.disc_id)
        //         .fetch_all(db),
        // )
        // .map_err(|e| format!("Failed to get tracks for disc {e}"))?;
        // result.push(DiscTs { disc, tracks });
    }
    return Ok(result);
}

#[tauri::command]
async fn get_tracks(state: tauri::State<'_, AppState>) -> Result<Vec<TrackSql>, String> {
    let guard = &state.state.lock().unwrap();
    let db = &guard.db;
    let mut statement = db.prepare_cached("SELECT * from Tracks").unwrap();

    let track_iterator = statement
        .query_map([], |row| {
            Ok(TrackSql {
                track_id: row.get(0)?,
                track_num: row.get(1)?,
                track_title: row.get(2)?,
                track_art_path: row.get(3)?,
                artist: row.get(4)?,
                track_path: row.get(5)?,
                album: row.get(7)?,
                disc: row.get(8)?,
            })
        })
        .unwrap();
    let mut track_list: Vec<TrackSql> = Vec::new();
    for track in track_iterator {
        track_list.push(track.unwrap());
    }

    drop(guard);

    return Ok(track_list);
}

async fn setup_db() -> Connection {
    let mut path = dirs::data_dir().unwrap();
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
    let conn = Connection::open(&path).unwrap();
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
    let base_folder = Path::new("/home/joseph/Music/.YALMP/albums.json");
    println!("Scanning started");
    if should_scan {
        let _ = scan(base_folder, &conn).await;
    }
    println!("scanning complete");
    return conn;
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
    let db = setup_db().await;
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
