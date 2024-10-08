import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

import './App.css';
import { Controls } from './Controls';
import reactLogo from './assets/react.svg';

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank" rel="noreferrer">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank" rel="noreferrer">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
      <Controls
        songs={[
          '/home/joseph/Music/Civilisation 5/Great Music/1-02-Brandenburg.mp3',
          '/home/joseph/Music/Civilisation 5/Great Music/1-08-Hungarian Dance No 5.mp3',
          '/home/joseph/Music/Homeworld 1 Remastered Original Soundtrack/01-08-Did Not Survive Interogation.flac',
          '/home/joseph/Music/Humankind/1-01-Humankind (Main Title).mp3',
          '/home/joseph/Music/Humankind/1-02-Oracles.mp3',
        ]}
      />
    </div>
  );
}

export default App;
