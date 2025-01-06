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
