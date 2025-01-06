import { invoke } from '@tauri-apps/api/core';

import { Disc, Track } from './types';

export const Controls = () => {
  function get_track_location(track: Track): string {
    return track.track_path;
  }

  function playlist_to_song_locations(tracks: Track[]): string[] {
    return tracks.map(get_track_location);
  }

  async function loadAllSongs() {
    let fake_binding: Track[] = [];
    await invoke('get_tracks').then((message) => {
      console.log('message is', message);
      fake_binding = message as Track[];
    });
    console.log('playlist is', fake_binding);
    await invoke('load_playlist', {
      newPlaylist: playlist_to_song_locations(fake_binding),
    });
  }

  async function loadFromAlbum(albumId: number) {
    let fake_binding: Disc[] = [];
    await invoke('get_discs', { albumId: albumId }).then((message) => {
      console.log('message is', message);
      fake_binding = message as Disc[];
    });
    let tracks_playlist: string[] = [];
    for (const disc of fake_binding) {
      tracks_playlist = tracks_playlist.concat(
        playlist_to_song_locations(disc.tracks),
      );
    }
    console.log(tracks_playlist);
    await invoke('load_playlist', { newPlaylist: tracks_playlist });
  }

  async function playSong() {
    await invoke('play_current_idx');
  }

  function pauseSong() {}

  return (
    <div>
      <button onClick={() => playSong()}>Play</button>
      <button onClick={() => loadAllSongs()}>Load Songs</button>
      <button onClick={() => loadFromAlbum(31)}>Load from Album</button>
      <button onClick={() => pauseSong()}>Pause</button>
    </div>
  );
};
