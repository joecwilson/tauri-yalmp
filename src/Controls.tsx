import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Howl } from 'howler';
import React from 'react';

export const Controls = () => {
  const currentTrack = '/home/joseph/Music/Humankind/1-08-Signs.mp3';
  //   playAudio();

  function getAudioUrl(track_file: string) {
    return convertFileSrc(track_file);
  }

  function playAudio() {
    const assetUrl = getAudioUrl(currentTrack);
    console.log(assetUrl);
    const sound = new Howl({
        src: [assetUrl]
    });

    sound.play();
  }

  return (
    <div>
      <audio src={getAudioUrl(currentTrack)} controls>
        The audio tag is not supported here
      </audio>
      <button onClick={() => playAudio()}></button>
    </div>
  );
};
