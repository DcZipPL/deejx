#![no_std]
#![no_main]

use alloc::vec::Vec;
use esp_hal::{clock::CpuClock, main, analog::adc::{AdcConfig, Attenuation}, Blocking, uart};

use esp_println as _;
use defmt::{info, println};
use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcChannel, AdcPin};
use esp_hal::uart::Uart;

extern crate alloc;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let analog_pin_35 = _peripherals.GPIO35;
    let analog_pin_32 = _peripherals.GPIO34;
    
    let mut adc1_config = AdcConfig::new();
    let mut pin_32 = adc1_config.enable_pin(analog_pin_32, Attenuation::_11dB);
    let mut pin_35 = adc1_config.enable_pin(analog_pin_35, Attenuation::_11dB);
    
    let mut adc1 = Adc::new(_peripherals.ADC1, adc1_config);

    let mut uart = Uart::new(_peripherals.UART0, uart::Config::default().with_baudrate(115_200)).unwrap();

    let mut prev_value_32 = 0;
    let mut prev_value_35 = 0;
    loop {
        let mut r_value_32 = prev_value_32;
        if let Ok(val) = read_stable_adc(&mut adc1, &mut pin_32) {
            r_value_32 = val;
            prev_value_32 = r_value_32;
        }

        let mut r_value_35 = prev_value_35;
        if let Ok(val) = read_stable_adc(&mut adc1, &mut pin_35) {
            r_value_35 = val;
            prev_value_35 = r_value_35;
        }
        
        let _ = uart.write(&build_packet(&[r_value_35, r_value_32]));
    }
}

fn read_stable_adc<PIN, ADCI>(adc: &mut Adc<ADCI, Blocking>, pin: &mut AdcPin<PIN, ADCI>) -> nb::Result<u16, ()>
where
    PIN: AdcChannel, ADCI: esp_hal::analog::adc::RegisterAccess
{
    let mut sum: u32 = 0;
    let samples = 256;

    for _ in 0..samples {
        let value = nb::block!(adc.read_oneshot(pin))? as u32;
        sum += value;
    }

    let average = (sum / samples) as u16;
    Ok(average / 2)
}

fn build_packet(sliders: &[u16]) -> Vec<u8> {
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