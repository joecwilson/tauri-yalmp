CREATE TABLE Discs (
  disc_id INTEGER PRIMARY KEY,
  disc_num INTEGER,
  disc_title TEXT,
  disc_art_path TEXT,
  json_path TEXT,
  album INTEGER,
  FOREIGN KEY (album) REFERENCES Albums (album_id)
) STRICT;