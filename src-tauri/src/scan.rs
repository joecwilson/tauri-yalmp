use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{env::set_current_dir, fs, path::Path};

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
    album: String,
    tracks: Vec<String>,
    name: Option<String>,
    art: Option<String>,
    comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrackJson {
    title: String,
    album: String,
    disc: String,
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

pub async fn scan(base_folder: &Path, pool: &SqlitePool) -> anyhow::Result<()> {
    for file in fs::read_dir(base_folder).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        let mut dir_found = false;
        if path.is_dir() {
            for sub_file in fs::read_dir(&path)? {
                let sub_file = sub_file?;
                let sub_path = sub_file.path();
                if sub_path.is_dir() {
                    dir_found = true;
                }
            }
            let _ = add_album_to_db(&path, pool).await;
            if dir_found {
                let _ = Box::pin(scan(&path, pool)).await?;
            }
        }
    }
    return Ok(());
}

async fn add_album_to_db(album: &Path, pool: &SqlitePool) -> anyhow::Result<()> {
    set_current_dir(album)?;
    let mut conn = pool.acquire().await?;
    let json_path = Path::new("album.json");
    let json_contents = fs::read_to_string(json_path)?;
    let album_contents: AlbumJson = serde_json::from_str(&json_contents).unwrap();

    // Now we can actually add it to the database
    let id = sqlx::query(
        r#"
    INSERT into Albums (title, album_artist, album_art_path)
    VALUES ( ?1, ?2, ?3)
    "#,
    )
    .bind(&album_contents.name)
    .bind(&album_contents.artist)
    .bind(&album_contents.art)
    .execute(&mut *conn)
    .await
    .unwrap()
    .last_insert_rowid();

    let _ = conn.close().await?;

    let mut disc_counter = 1;
    for disc in &album_contents.discs {
        let disc_path = Path::new(&disc);
        let _ = add_disc_to_db(&disc_path, id, disc_counter, pool)
            .await?;
        disc_counter += 1;
    }
    println!("{album_contents:?}");
    return Ok(());
}

async fn add_disc_to_db(
    disc: &Path,
    album_id: i64,
    disc_num: i32,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    let json_contents = fs::read_to_string(disc).unwrap();
    let disc_contents: DiscJson = serde_json::from_str(&json_contents).unwrap();

    let mut conn = pool.acquire().await?;
    let id = sqlx::query(
        r#"
    INSERT into Discs (disc_title, disc_num, disc_art_path, album)
    VALUES ( ?1, ?2, ?3, ?4)
    "#,
    )
    .bind(&disc_contents.name)
    .bind(disc_num)
    .bind(&disc_contents.art)
    .bind(album_id)
    .execute(&mut *conn)
    .await
    .unwrap()
    .last_insert_rowid();

    let _ = conn.close().await?;

    let mut track_num = 1;
    for track in &disc_contents.tracks {
        let track_path = Path::new(&track);
        let _ = add_track_to_db(&track_path, album_id, id, track_num, pool)
            .await?;
        track_num += 1;
    }
    return Ok(());
}

async fn add_track_to_db(
    track: &Path,
    album_id: i64,
    disc_id: i64,
    track_num: i32,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    let json_contents = fs::read_to_string(track)?;
    let track_contents: TrackJson = serde_json::from_str(&json_contents).unwrap();

    let mut conn = pool.acquire().await?;
    let _ = sqlx::query(
        r#"
    INSERT into Tracks (track_num, track_title, track_art_path, artist, track_path, album, disc)
    VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7)
    "#,
    )
    .bind(track_num)
    .bind(track_contents.title)
    .bind(track_contents.art)
    .bind(track_contents.artist)
    .bind(track_contents.path)
    .bind(album_id)
    .bind(disc_id)
    .execute(&mut *conn)
    .await
    .unwrap();

    let _ = conn.close().await.unwrap();
    return Ok(());
}
