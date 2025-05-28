use alloc::vec::Vec;

pub(crate) fn build(sliders: &[u16]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(4 + sliders.len() * 2);
    packet.push(0xAA);               // Header
    packet.push(sliders.len() as u8);      // Count

    for &val in sliders {
        packet.extend(&val.to_le_bytes()); // Append each u16
    }

    let checksum = packet[1..].iter().fold(0u8, |acc, &b| acc ^ b); // exclude header
    packet.push(checksum);
    packet
}