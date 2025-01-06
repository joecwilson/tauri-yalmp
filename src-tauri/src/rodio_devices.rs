use rodio::{
    cpal::{self, host_from_id, traits::HostTrait},
    Device, DeviceTrait, OutputStream, Sink,
};
use std::string::String;

use crate::app_state::{AppState, SendStream};
use std::vec::Vec;

#[tauri::command]
pub async fn list_devices() -> Result<Vec<String>, String> {
    let host_ids = cpal::platform::available_hosts();

    let mut result_vec = Vec::new();
    for host_id in host_ids {
        let host = host_from_id(host_id).expect("avaliable platform to be useable");
        println!("host: {}", host_id.name());
        for device in host
            .output_devices()
            .expect("At least 1 output device should be avalable")
        {
            result_vec.push(device.name().expect("A human readable name to exist"));
            println!("{}", device.name().expect("A name to exist"));
        }
    }

    return Ok(result_vec);
}

#[tauri::command]
pub async fn switch_device(
    state: tauri::State<'_, AppState>,
    device_name: String,
) -> Result<(), String> {
    let host_ids = cpal::platform::available_hosts();

    for host_id in host_ids {
        let host = host_from_id(host_id).expect("avaliable platform to be useable");
        for device in host
            .output_devices()
            .expect("At least 1 output device should be avalable")
        {
            if device.name().unwrap() == device_name {
                switch_device_given_device(&state, device).await.unwrap()
            }
        }
    }

    Ok(())
}

async fn switch_device_given_device(
    state: &tauri::State<'_, AppState>,
    device: Device,
) -> anyhow::Result<()> {
    let mut guard = state.state.lock().await;
    guard.current_sink.clear();
    guard.current_playing_counter = None;
    guard.requested_playing_counter = Some(0);

    let (stream, stream_handle) = OutputStream::try_from_device(&device)?;
    let new_sink = Sink::try_new(&stream_handle)?;

    let send_stream = SendStream(stream);
    guard.current_sink = new_sink;
    guard.current_sink_output_handle = Some(stream_handle);
    guard.current_sink_output_stream = Some(send_stream);
    Ok(())
}
