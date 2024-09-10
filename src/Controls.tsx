import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Howl } from 'howler';
import { useEffect, useState } from 'react';

interface playlistProps {
  songs: string[];
}

export const Controls = ({ songs }: playlistProps) => {
  const [currentPos, setCurrentPos] = useState<number>(getStartingPos());
  const [playing, setPlaying] = useState<Howl>(getHowl(songs[currentPos]));
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

    playing.on('end', () => {
      playNextSong();
    });
  }, []);

  function playNextSong() {
    if (currentPos === songs.length) {
      return;
    }
    setCurrentPos(currentPos + 1);
    setPlaying(getHowl(songs[currentPos]));
    playSong();
  }

  function getHowl(track: string): Howl {
    const assetUrl = convertFileSrc(track);
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
