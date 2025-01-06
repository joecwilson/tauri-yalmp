use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
struct AlbumJson {
    name: String,
    artist: String,
    discs: Vec<String>,
    art: Option<String>,
    copyright: Option<String>,
    comment: Option<String>,
    years: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DiscJson {
    tracks: Vec<String>,
    name: Option<String>,
    art: Option<String>,
    comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrackJson {
    title: String,
    path: String,
    art: Option<String>,
    artist: Option<String>,
    composer: Option<String>,
    rating: Option<i32>,
    lyric_path_raw: Option<String>,
    lyric_path_transliterated: Option<String>,
    lyric_path_translated: Option<String>,
    has_lyrics: Option<bool>,
    has_vocals: Option<bool>,
    synchronized_lyrics_path: Option<String>,
}

pub fn scan(albums_file: &Path, conn: &Connection) -> anyhow::Result<()> {
    conn.execute(include_str!("../migrations/create_album_table.sql"), ())
        .unwrap();
    conn.execute(include_str!("../migrations/create_disc_table.sql"), ())
        .unwrap();
    conn.execute(include_str!("../migrations/create_track_table.sql"), ())
        .unwrap();
    let albums_string = fs::read_to_string(albums_file).unwrap();
    let album_vec: Vec<String> = serde_json::from_str(&albums_string)?;
    for album in album_vec {
        let album_path = Path::new(&album);
        let _ = add_album_to_db(&album_path, conn)?;
    }
    return Ok(());
}

fn add_album_to_db(album: &Path, conn: &Connection) -> anyhow::Result<()> {
    let json_contents = fs::read_to_string(album)?;
    let album_contents: AlbumJson = serde_json::from_str(&json_contents)?;
    let album_as_string = album.to_string_lossy();

    // Now we can actually add it to the database
    conn.execute(
        "INSERT into Albums (title, album_artist, album_art_path, json_path)
    VALUES ( ?1, ?2, ?3, ?4)",
        (
            &album_contents.name,
            &album_contents.artist,
            &album_contents.art,
            album_as_string,
        ),
    )?;
    let id = conn.last_insert_rowid();

    let mut disc_counter = 1;
    for disc in &album_contents.discs {
        let disc_path = Path::new(&disc);
        let _ = add_disc_to_db(&disc_path, id, disc_counter, conn)?;
        disc_counter += 1;
    }
    println!("{album_contents:?}");
    return Ok(());
}

fn add_disc_to_db(
    disc: &Path,
    album_id: i64,
    disc_num: i32,
    conn: &Connection,
) -> anyhow::Result<()> {
    let json_contents = fs::read_to_string(disc)?;
    let disc_contents: DiscJson = serde_json::from_str(&json_contents)?;
    let disc_as_str = disc.to_string_lossy();

    // let mut conn = pool.acquire().await?;

    conn.execute(
        "INSERT into Discs (disc_title, disc_num, disc_art_path, album, json_path)
    VALUES ( ?1, ?2, ?3, ?4, ?5)",
        (
            &disc_contents.name,
            disc_num,
            &disc_contents.art,
            album_id,
            &disc_as_str,
        ),
    )?;

    let id = conn.last_insert_rowid();

    let mut track_num = 1;
    for track in &disc_contents.tracks {
        let track_path = Path::new(&track);
        let _ = add_track_to_db(&track_path, album_id, id, track_num, conn)?;
        track_num += 1;
    }
    return Ok(());
}

fn add_track_to_db(
    track: &Path,
    album_id: i64,
    disc_id: i64,
    track_num: i32,
    conn: &Connection,
) -> anyhow::Result<()> {
    let json_contents = fs::read_to_string(track)?;
    let track_contents: TrackJson = serde_json::from_str(&json_contents)?;
    let track_as_str = track.to_string_lossy();

    // let mut conn = pool.acquire().await?;
    conn.execute("INSERT into Tracks (track_num, track_title, track_art_path, artist, track_path, album, disc, json_path)
    VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", (&track_num, &track_contents.title, &track_contents.art, &track_contents.artist, &track_contents.path,  &album_id, &disc_id, &track_as_str)).unwrap();

    return Ok(());
}
