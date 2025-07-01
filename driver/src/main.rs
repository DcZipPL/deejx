mod packets;
mod audio;
mod config;
mod midi;

use std::{env, thread};
use std::time::Duration;
use log::{debug, error, info, warn};
use serialport::SerialPort;
use crate::audio::{AudioControl, RAW_MAX};
use crate::config::{get_config_path, prepare_config, Mapping};
use pretty_env_logger::env_logger;

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
                    controller.set_master_volume_by_index(*master, sliders[i]);
                }
                Mapping::Device { device, .. } => {
                    controller.set_master_volume_by_name(device, sliders[i]);
                }
                Mapping::App {app, ..} => {
                    controller.set_app_volume_by_name(app.as_str(), sliders[i]);
                }
                Mapping::Midi {midi, ..} => {
                    error!("Cannot change value, MIDI not supported yet.");
                },
                &Mapping::Unmapped { .. } => {
                    error!("Cannot change value, Unmapped not supported yet.");
                },
            }
        }
    }
}

fn create_serial(port: &str, baud_rate: u32, timeout: u64) -> serialport::Result<Box<dyn SerialPort>> {
    let timeout = Duration::from_millis(timeout);

    serialport::new(port, baud_rate)
        .timeout(timeout)
        .open()
}

fn main() {
    env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init();
    debug!("@{}:{}({})@", env!("CARGO_PKG_VERSION"), env::consts::OS, env::consts::ARCH);
    info!("Starting driver...\nWelcome to {}, control volumes like a DJ!\nIf you encounter any problem, you can make issue on https://github.com/DcZipPL/deejx\nRemember to read FAQ before submitting an issue.\nThank you for using deejx!", env!("CARGO_PKG_NAME"));

    let path = get_config_path().expect("Failed to get config path");
    let (mut config, watcher) = prepare_config(&path).expect("Failed to initialise configuration watchers and reads");

    let mut controller = audio::get_controller();
    info!("Audio controller ({}) started!", controller.name());
    info!("Ready!");
    loop {
        let mut serial = create_serial(&config.serial, config.baud_rate, config.timeout);
        while serial.is_err() {
            warn!("Serial connection failed. Retrying...");
            thread::sleep(Duration::from_millis(500));
            serial = create_serial(&config.serial, config.baud_rate, config.timeout);
        }
        let mut serial = serial.unwrap();

        let mut previous_values = Vec::new();
        loop {
            // Main loop
            if let Some(sliders) = packets::read_packet(&mut serial) {
                iter_sliders(sliders, &mut previous_values, &mut controller, &config.mappings);
            }

            // Reload config
            let old_config = config.clone();
            config.update(&path, &watcher);
            if old_config.baud_rate != config.baud_rate
                || old_config.timeout != config.timeout
                || old_config.serial != config.serial {
                info!("Serial data changed in config. Hard-resetting serial...");
                break; // Go up to create serial
            }
        }
    }
}