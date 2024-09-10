import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Howl } from 'howler';
import React, { useEffect, useState } from 'react';

export const Controls = () => {
  const [currentTrack, setCurrentTrack] = useState(
    '/home/joseph/Music/Humankind/1-08-Signs.mp3',
  );
  const [playing, setPlaying] = useState(getHowl(currentTrack));
  const [duration, setDuration] = useState(0);
  const [seekTime, setSeekTime] = useState(0);

  useEffect(() => {
    setInterval(() => {
      setSeekTime(playing.seek());
      setDuration(playing.duration());
    }, 1000);
  }, []);

  function getAudioUrl(track_file: string) {
    return convertFileSrc(track_file);
  }

  function getHowl(track: string): Howl {
    const assetUrl = getAudioUrl(track);
    console.log(assetUrl);
    return new Howl({
      src: [assetUrl],
    });
  }

  function playSong() {
    console.log(playing);
    playing.play();
  }

  function pauseSong() {
    console.log(playing);
    console.log(playing.seek());
    playing.pause();
  }

  function renderSeekTimeDuration() {
    if (!seekTime || !duration) {
      return;
    }

    return (
      <div>
        <p>{seekTime ?? 0}</p>
        <p>{duration ?? 0}</p>
      </div>
    );
  }

  return (
    <div>
      <button onClick={() => playSong()}>Play</button>
      <button onClick={() => pauseSong()}>Pause</button>
      {renderSeekTimeDuration()}
    </div>
  );
};
