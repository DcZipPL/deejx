use log::warn;
use serialport::SerialPort;

// Packet: [0xAA] [count] [pin0, pin2, pin3, ...] [checksum]
pub fn read_packet(serial: &mut Box<dyn SerialPort>) -> Option<Vec<u16>> {
    let mut header = [0u8; 1];

    loop {
        // Wait for header
        serial.read_exact(&mut header).ok()?;
        if header[0] != 0xAA {
            continue;
        }

        // Check amount of sliders
        let mut count_buf = [0u8; 1];
        serial.read_exact(&mut count_buf).ok()?;
        let count = count_buf[0] as usize;

        // Read payload
        let mut payload = vec![0u8; 2 * count + 1]; // values + checksum
        serial.read_exact(&mut payload).ok()?;

        let checksum = std::iter::once(count_buf[0])
            .into_iter()
            .chain(payload[..2*count].iter().copied())
            .fold(0u8, |acc, b| acc ^ b);

        if checksum != payload[2 * count] {
            warn!("Checksum mismatch, skipping...");
            continue;
        }

        let values: Vec<u16> = payload[..2 * count]
            .chunks(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();

        return Some(values);
    }
}
