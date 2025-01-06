import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';

export const Devices = () => {
  const [devices, setDevices] = useState<string[]>([]);

  async function get_devices_list() {
    console.log('get_devices_called');
    await invoke('list_devices').then((new_devices) => {
      console.log(new_devices);
      setDevices(new_devices as string[]);
    });
  }

  async function set_device(device: string) {
    await invoke('switch_device', { deviceName: device });
  }

  function renderDeviceList() {
    const device_items = devices.map((device) => (
      <button key={device} onClick={() => set_device(device)}>
        {device}
      </button>
    ));
    return <div>{device_items}</div>;
  }

  return (
    <>
      <button onClick={() => get_devices_list()}> List Devices</button>
      {renderDeviceList()}
    </>
  );
};
