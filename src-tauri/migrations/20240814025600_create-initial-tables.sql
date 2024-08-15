-- Add migration script here
CREATE TABLE Albums (
  AlbumId INTEGER PRIMARY KEY,
  Title TEXT,
  AlbumArtist TEXT,
  AlbumArtPath TEXT
) STRICT;

CREATE TABLE Discs (
  DiscId INTEGER PRIMARY KEY,
  DiscNum INTEGER,
  DiscTitle TEXT,
  DiscArtPath TEXT,
  Album INTEGER,
  FOREIGN KEY (Album) REFERENCES Albums (AlbumId)
) STRICT;

CREATE TABLE Tracks (
  TrackId INTEGER PRIMARY KEY,
  TrackNum INTEGER,
  TrackTitle TEXT,
  TrackArtPath TEXT,
  Artist TEXT,
  TrackPath TEXT,
  Album INTEGER,
  Disc INTEGER,
  FOREIGN KEY (Album) REFERENCES Albums (AlbumId),
  FOREIGN KEY (Disc) REFERENCES Discs (DiscId)
) STRICT;
