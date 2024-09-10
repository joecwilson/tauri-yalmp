import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Howl } from 'howler';
import React, { useState } from 'react';

export const Controls = () => {
  const [currentTrack, setCurrentTrack] = useState('/home/joseph/Music/Humankind/1-08-Signs.mp3');
  const [playing, setPlaying] = useState(getHowl(currentTrack));

  function getAudioUrl(track_file: string) {
    return convertFileSrc(track_file);
  }

  function getHowl(track: string): Howl {
    const assetUrl = getAudioUrl(track);
    console.log(assetUrl);
    return new Howl({
        src: [assetUrl]
    });
  }

  function playSong() {
    console.log(playing);
    playing.play();
  }


  function pauseSong() {
    console.log(playing);
    console.log(playing.seek())
    playing.pause();
  }


  return (
    <div>
      <button onClick={() => playSong()}>Play</button>
      <button onClick={() => pauseSong()}>Pause</button>
      <p>{playing.seek()}</p>
      <p>{playing.duration()}</p>
    </div>
  );
};
