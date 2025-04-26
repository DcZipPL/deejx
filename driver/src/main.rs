mod packets;
mod audio;

use std::time::Duration;
use crate::audio::{AudioControl, RAW_MAX};

enum Mapping<'a> {
    Master(u32),
    App(&'a str),
    Midi(u16),
}

const STEP_SIZE: i32 = 64;

fn start_reading() {
    let mut controller = audio::get_controller();
    let temp_mapping = [Mapping::Master(0), Mapping::App("spotify")];
    
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
            iter_sliders(sliders, &mut previous_values, &mut controller, &temp_mapping);
        }
    }
}

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
            match mappings[i] {
                Mapping::Master(device) => {
                    controller.set_master_volume(device, sliders[i]);
                }
                Mapping::App(name) => {
                    controller.set_app_volume_by_name(name, sliders[i]);
                }
                Mapping::Midi(value) => {
                    unimplemented!()
                }
            }
        }
    }
}

fn main() {
    start_reading();
}