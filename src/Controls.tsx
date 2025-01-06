import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import { Track } from './types';

interface playlistProps {
  songs: string[];
}

export const Controls = ({ songs }: playlistProps) => {
  const [playlist, setPlaylist] = useState<Track[]>([]);

  function get_track_location(track: Track) {
    return track.track_path;
  }

  function playlist_to_song_locations(tracks: Track[]) {
    return tracks.map(get_track_location)
  }


  async function loadAllSongs() {
    let fake_binding: Track[] = []
    await invoke('get_tracks').then((message) =>  {
      setPlaylist(message as Track[]);
      console.log("message is", message)
      fake_binding = message as Track[]
    });
    console.log("playlist is", fake_binding)
    await invoke('load_playlist', {newPlaylist: playlist_to_song_locations(fake_binding)});
  }


  async function playSong() {
    await invoke("play_current_idx");
  }


  function pauseSong() {
  }




  return (
    <div>
      <button onClick={() => playSong()}>Play</button>
      <button onClick={() => loadAllSongs()}>Load Songs</button>
      <button onClick={() => pauseSong()}>Pause</button>
    </div>
  );
};
