mod packets;
mod audio;
mod config;
mod midi;

use std::time::Duration;
use serialport::SerialPort;
use crate::audio::{AudioControl, RAW_MAX};
use crate::config::{read_config, Mapping};

const STEP_SIZE: i32 = 64;

fn iter_sliders(sliders: Vec<u16>, previous_values: &mut Vec<u16>, controller: &mut Box<impl AudioControl>, mappings: &[Mapping]) {
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
            match &mappings[i] {
                Mapping::Master {master, ..} => {
                    controller.set_master_volume(*master, sliders[i]);
                }
                Mapping::App {app, ..} => {
                    controller.set_app_volume_by_name(app.as_str(), sliders[i]);
                }
                Mapping::Midi {midi, ..} => {
                    todo!()
                },
                &Mapping::Unmapped { .. } => todo!()
            }
        }
    }
}

fn create_serial() -> Box<dyn SerialPort> {
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115_200;
    let timeout = Duration::from_millis(20000);

    serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()
        .expect("Failed to open port")
}

fn main() {
    let mapping = read_config();
    let Ok(mapping) = mapping else {
        eprintln!("Configuration maybe invalid!");
        eprintln!("{}", mapping.unwrap_err());
        std::process::exit(1);
    };

    let mut controller = audio::get_controller();
    let mut serial = create_serial();

    let mut previous_values = Vec::<u16>::new();
    loop {
        if let Some(sliders) = packets::read_packet(&mut serial) {
            iter_sliders(sliders, &mut previous_values, &mut controller, &mapping.mappings);
        }
    }
}