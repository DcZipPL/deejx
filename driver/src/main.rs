mod packets;

use std::io::{BufRead, BufReader};
use std::time::Duration;
use libpulse_binding::volume::{ChannelVolumes, Volume};
use pulsectl::controllers::{DeviceControl, SinkController};

const RAW_MAX: f32 = 1023.;
const DEAD_ZONE: f32 = 0.005;
const MINIMUM: f32 = 0.;
const MAXIMUM: f32 = 100.;

fn bar(values: &[u16], pulse: &mut SinkController, volumes: &mut ChannelVolumes) {
    let data = values.iter()
        .map(|x| f32::from(*x))
        .map(|x| x / RAW_MAX * (MAXIMUM + (DEAD_ZONE * 2.)) - DEAD_ZONE)
        .map(|x| x.max(MINIMUM).min(MAXIMUM))
        .collect::<Vec<f32>>();

    let percent = data.first().unwrap_or(&0.);
    let volume: u32 = ((percent / 100.0) * 65536.0).round() as u32;
    
    pulse.set_device_volume_by_index(0, volumes.set(2, Volume(volume)))
}

fn start_reading() {
    let mut controller = SinkController::create().unwrap();
    let mut channels = controller.get_device_by_index(0).unwrap().volume;
    
    
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;
    let timeout = Duration::from_millis(20000);

    let serial = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()
        .expect("Failed to open port");

    let mut previous_values = Vec::<u16>::new();
    let mut reader = BufReader::new(serial);
    let mut buffer = Vec::<u8>::new();

    loop {
        buffer.clear(); // empty the buffer each time
        let read_result = reader.read_until(b'\n', &mut buffer);
        match read_result {
            Ok(0) => {
                // Stream ended / timeout
                eprintln!("No data received (timeout or disconnected)");
                break;
            }
            Ok(_) => {
                //process_volume_data();
            }
            Err(e) => {
                eprintln!("Serial read error: {:?}", e);
                break;
            }
        }
    }
}

/*fn process_volume_data(previous_values: Vec<u16>, data: &[u16]) {
    // Convert buffer to string safely
    if let Ok(line) = std::str::from_utf8(&buffer) {
        let trimmed = line.trim();

        let values: Result<Vec<u16>, _> = trimmed
            .split('\t')
            .map(str::trim)
            .map(|s| s.parse::<u16>())
            .collect();

        match values {
            Ok(values) if values != previous_values => {
                previous_values = values.clone();
                bar(&values, &mut controller, &mut channels);
            },
            Ok(_) => {}, // same as previous, ignore
            Err(_) => eprintln!("Invalid line: {:?}", trimmed),
        }
    } else {
        eprintln!("Invalid UTF-8 data: {:?}", buffer);
    }
}*/

fn main() {
    start_reading();
}
