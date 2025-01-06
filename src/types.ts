export type Album = {
  album_id: number;
  title: string;
  album_artist: string;
  album_art_path?: string;
};

export type DiscMetadata = {
  disc_id: number;
  disc_num: number;
  disc_title?: string;
  disc_art?: string;
  album: number;
};

export type Track = {
  track_id: number;
  track_num: number;
  track_title: string;
  track_art_path?: string;
  artist: string;
  track_path: string;
  album: number;
  disc: number;
};

export type Disc = {
  disc: DiscMetadata;
  tracks: Track[];
};
