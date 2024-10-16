-- Add migration script here
CREATE TABLE Albums (
  album_id INTEGER PRIMARY KEY,
  title TEXT,
  album_artist TEXT,
  album_art_path TEXT,
  json_path TEXT,
) STRICT;

CREATE TABLE Discs (
  disc_id INTEGER PRIMARY KEY,
  disc_num INTEGER,
  disc_title TEXT,
  disc_art_path TEXT,
  json_path TEXT,
  album INTEGER,
  FOREIGN KEY (album) REFERENCES Albums (album_id)
) STRICT;

CREATE TABLE Tracks (
  track_id INTEGER PRIMARY KEY,
  track_num INTEGER,
  track_title TEXT,
  track_art_path TEXT,
  artist TEXT,
  track_path TEXT,
  json_path TEXT,
  album INTEGER,
  disc INTEGER,
  FOREIGN KEY (album) REFERENCES Albums (album_id),
  FOREIGN KEY (disc) REFERENCES Discs (disc_id)
) STRICT;
