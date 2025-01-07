import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import React, { useState } from 'react';

import { Disc, NewSong, Track } from './types';

export const Controls = () => {
  const [tracks, setTracks] = useState<Track[]>([]);
  const [track_idx, setTrackIdx] = useState(0);

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
    setTracks(fake_binding);
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
    let track_location_playlist: string[] = [];
    let track_playlist: Track[] = [];
    for (const disc of fake_binding) {
      track_location_playlist = track_location_playlist.concat(
        playlist_to_song_locations(disc.tracks),
      );
      track_playlist = track_playlist.concat(disc.tracks);
    }
    console.log(track_location_playlist);
    setTracks(track_playlist);
    await invoke('load_playlist', { newPlaylist: track_location_playlist });
  }

  async function playSong() {
    await invoke('play_current_idx');
  }

  async function pauseSong() {
    await invoke('pause_song');
  }
  async function scan_music() {
    await invoke('scan_command');
  }

  listen<NewSong>('new_song', (event) => {
    setTrackIdx(event.payload.song_idx);
  });

  async function handleSeekSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const form = e.target;
    const formData = new FormData(form);

    const formJson = Object.fromEntries(formData.entries());

    console.log('form json  is ', formJson);

    await invoke('seek_song', { milliseconds: Number(formJson['seek']) });
  }

  function renderSeekDuration() {
    if (tracks.length === 0) {
      return <p>No playlist loaded</p>;
    }
    return (
      <>
        <h3>Duration</h3>
        <p>{tracks[track_idx].duration}</p>
        <form method="post" onSubmit={handleSeekSubmit}>
          <label htmlFor="seekInput">
            Seek Time: Should be less than duration
            <input name="seek" type="number" id="seekInput" />
            <button type="submit"> Seek</button>
          </label>
        </form>
      </>
    );
  }

  return (
    <div>
      <button onClick={() => playSong()}>Play</button>
      <button onClick={() => loadAllSongs()}>Load Songs</button>
      <button onClick={() => loadFromAlbum(2)}>Load from Album</button>
      <button onClick={() => pauseSong()}>Pause</button>
      <button onClick={() => scan_music()}>Scan</button>
      {renderSeekDuration()}
    </div>
  );
};
