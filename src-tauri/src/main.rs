// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_state::SendStream;
use tokio::sync::Mutex;

use crate::app_state::{AppState, InteriorAppState};
use crate::playlist::{load_playlist, set_playlist_idx};
use crate::rodio_devices::{list_devices, switch_device};
use crate::scan::scan;
use dirs;
use rodio::{OutputStream, Sink};
use rusqlite::{Connection, Result};
use tauri::Manager;

mod app_state;
mod play;
mod playlist;
mod rodio_devices;
mod scan;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct AlbumSql {
    album_id: i64,
    title: String,
    album_artist: String,
    album_art_path: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DiscSql {
    disc_id: i64,
    disc_num: i32,
    disc_title: Option<String>,
    disc_art_path: Option<String>,
    album: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TrackSql {
    track_id: i64,
    track_num: i32,
    track_title: String,
    track_art_path: Option<String>,
    artist: String,
    track_path: String,
    album: i64,
    disc: i64,
    duration: i32,
}

#[tauri::command]
async fn get_albums(state: tauri::State<'_, AppState>) -> Result<Vec<AlbumSql>, String> {
    let guard = &state.state.lock().await;
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
    let guard = &state.state.lock().await;
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
                    duration: row.get(7)?,
                    album: row.get(8)?,
                    disc: row.get(9)?,
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
    }
    return Ok(result);
}

#[tauri::command]
async fn get_tracks(state: tauri::State<'_, AppState>) -> Result<Vec<TrackSql>, String> {
    let guard = &state.state.lock().await;
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
                duration: row.get(7)?,
                album: row.get(8)?,
                disc: row.get(9)?,
            })
        })
        .unwrap();
    let mut track_list: Vec<TrackSql> = Vec::new();
    for track in track_iterator {
        track_list.push(track.unwrap());
    }

    return Ok(track_list);
}

fn setup_db() -> Connection {
    let mut path = dirs::data_dir().unwrap();
    path.push("dev.josephwilson.yalmp");
    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };
    path.push("db.sqlite");
    let result = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path);
    let mut should_scan = false;
    let conn = Connection::open(&path).unwrap();
    match result {
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
    let mut base_folder = dirs::audio_dir().unwrap();
    base_folder.push(".yalmp");
    base_folder.push("albums.json");
    println!("Scanning started");
    if should_scan {
        let _ = scan(&base_folder, &conn).unwrap();
    }
    println!("scanning complete");
    return conn;
}

#[tokio::main]
async fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_albums,
            get_discs,
            get_tracks,
            load_playlist,
            set_playlist_idx,
            play::play_current_idx,
            list_devices,
            switch_device,
            play::pause_song,
            play::get_current_location,
            play::seek_song,
            scan::scan_command
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    let db = setup_db();
    let (stream, stream_handle) = OutputStream::try_default()
        .map_err(|_| format!("Failed to open stream"))
        .unwrap();
    let new_sink = Sink::try_new(&stream_handle)
        .map_err(|_| format!("Failed to create new audio sink"))
        .unwrap();
    new_sink.set_volume(0.1);
    let send_stream = SendStream(stream);
    let interior_app_state = InteriorAppState {
        db,
        current_playlist: Vec::new(),
        current_playlist_idx: 0,
        current_sink: new_sink,
        current_sink_output_stream: Some(send_stream),
        current_sink_output_handle: Some(stream_handle),
        current_playing_counter: 0,
        requested_playing_counter: 0,
        stopped: true,
    };
    let state = Mutex::new(interior_app_state);
    app.manage(AppState { state });
    app.run(|_, _| {});
}
