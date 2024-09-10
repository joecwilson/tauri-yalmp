import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Howl } from 'howler';
import { useEffect, useState } from 'react';

interface playlistProps {
  songs: string[];
}

export const Controls = ({ songs }: playlistProps) => {
  const [currentPos, setCurrentPos] = useState<number>(() =>  getStartingPos());
  const [playing, setPlaying] = useState<Howl>(() =>  getHowl(songs[0]));
  const [duration, setDuration] = useState<number>(0);
  const [seekTime, setSeekTime] = useState<number>(0);

  function getStartingPos(): number {
    if (songs.length === 0) {
      throw new Error('Empty Songs given');
    }
    return 0;
  }

  useEffect(() => {
    setInterval(() => {
      setSeekTime(playing.seek());
      setDuration(playing.duration());
    }, 1000);

    playing.once('end', () => {
      playNextSong();
    });
  }, []);

  function playNextSong() {
    if (currentPos === songs.length) {
      return;
    }
    console.log("Starting currentPos = ", currentPos);
    setCurrentPos((prevPos) => prevPos + 1);
    console.log("after updating currentPos = ", currentPos + 1);
    const newHowl = getHowl(songs[currentPos + 1]);
    setPlaying(newHowl);
    playSong(newHowl);

    newHowl.once('end', () => {
    playNextSong();
    });
  }

  function getHowl(track: string): Howl {
    console.log("Song to play is", track);
    const assetUrl = convertFileSrc(track);
    return new Howl({
      src: [assetUrl],
    });
  }

  function playSong(currentHowl: Howl) {
    console.log(currentHowl);
    currentHowl.play();
  }

  function pauseSong(currentHowl: Howl) {
    currentHowl.pause();
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
      <button onClick={() => playSong(playing)}>Play</button>
      <button onClick={() => pauseSong(playing)}>Pause</button>
      {renderSeekTimeDuration()}
    </div>
  );
};
