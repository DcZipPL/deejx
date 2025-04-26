mod packets;

use std::time::Duration;
use libpulse_binding::volume::{ChannelVolumes, Volume};
use pulsectl::controllers::{AppControl, DeviceControl, SinkController};

const RAW_MAX: f32 = 2047.;
const STEP_SIZE: i32 = 64;
const MINIMUM: f32 = 0.;
const MAXIMUM: f32 = 100.;

fn change_volume(values: &[u16], pulse: &mut SinkController, volumes: &mut ChannelVolumes) {
    let data = values.iter()
        .map(|x| f32::from(*x))
        .map(|x| x / RAW_MAX * MAXIMUM)
        .map(|x| x.max(MINIMUM).min(MAXIMUM))
        .collect::<Vec<f32>>();

    let percent = data.first().unwrap_or(&0.);
    let volume: u32 = ((percent / 100.0) * 65536.0).round() as u32;
    
    pulse.set_device_volume_by_index(0, volumes.set(2, Volume(volume)))
}

fn set_app_volume(pulse: &mut SinkController, change: u16){
    if let Ok(apps) = pulse.list_applications() {
        for app in apps {
            if app.name.is_some_and(|s| s.to_lowercase().contains("spotify")) {
                let app_volume = app.volume.avg().0 as f32 / 65536.;
                let volume: f32 = (change as f32 / RAW_MAX * 100.0).round() / 100.0;
                let volume_change = volume - app_volume;
                println!("left: {:?}, right: {:?}, change: {}", app.volume.get()[0], app.volume.get()[1], volume_change);
                if volume_change > 0. {
                    pulse.increase_app_volume_by_percent(app.index, volume_change.abs() as f64);
                } else {
                    pulse.decrease_app_volume_by_percent(app.index, volume_change.abs() as f64);
                }
            }
        }
    }
}

fn start_reading() {
    let mut controller = SinkController::create().unwrap();
    let mut channels = controller.get_device_by_index(0).unwrap().volume;
    
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115_200;
    let timeout = Duration::from_millis(20000);

    let mut serial = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()
        .expect("Failed to open port");

    let mut previous_values = Vec::<u16>::new();

    loop {
        if let Some(sliders) = packets::read_packet(&mut serial) {
            iter_sliders(sliders, &mut previous_values, &mut controller, &mut channels);
        }
    }
}

fn iter_sliders(sliders: Vec<u16>, previous_values: &mut Vec<u16>, controller: &mut SinkController, channels: &mut ChannelVolumes) {
    // Prepare initial array
    if previous_values.len() != sliders.len() {
        *previous_values = sliders.clone();
    }

    // Go through sliders
    for i in 0..sliders.len() {
        let value = sliders[i] as i32;
        let prev_value = previous_values[i] as i32;

        if ((value - prev_value).abs() >= STEP_SIZE) /* We don't want unstable values and reduce calls to shell */
            || ((value < 1 || value > RAW_MAX as i32 - 1) && value != prev_value) /* Safety net for extreme values */ {

            previous_values[i] = sliders[i];
            if i == 0 {
                println!("a: {} -> {}", previous_values[i], value);
                change_volume(&[sliders[i]], controller, channels); // TODO: impl other way
            } else {
                println!("b: {} -> {}", previous_values[i], value);
                set_app_volume(controller, sliders[i]);
            }
        }
    }
}

fn main() {
    start_reading();
}