#![no_std]
#![no_main]

mod packet;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use esp_hal::{clock::CpuClock, main, analog::adc::{AdcConfig, Attenuation}, Blocking, uart};

use esp_println as _;
use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcChannel, AdcPin};
use esp_hal::gpio::GpioPin;
use esp_hal::peripherals::ADC1;
use esp_hal::uart::Uart;

extern crate alloc;

#[main]
fn main() -> ! {
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let analog_pin_35 = _peripherals.GPIO35;
    let analog_pin_32 = _peripherals.GPIO34;
    
    let mut adc1_config = AdcConfig::new();
    let mut pin_32 = adc1_config.enable_pin(analog_pin_32, Attenuation::_11dB);
    let mut pin_35 = adc1_config.enable_pin(analog_pin_35, Attenuation::_11dB);
    
    let mut adc1 = Adc::new(_peripherals.ADC1, adc1_config);
    
    let mut pin_32_r = ReadableInput {
        adc: &adc1,
        pin: pin_32
    };

    let mut pin_35_r = ReadableInput {
        adc: &adc1,
        pin: pin_35
    };

    let mut enabled_pins: Vec<Box<dyn Readable>> = vec![Box::new(pin_32_r), Box::new(pin_35_r)];

    let mut uart = Uart::new(_peripherals.UART0, uart::Config::default().with_baudrate(115_200)).unwrap();

    let mut prev_values = vec![0, 0];
    loop {
        /*let mut r_value_32 = prev_values[0];
        if let Ok(val) = read_stable_adc(&mut adc1, &mut pin_32) {
            r_value_32 = val;
            prev_values[0] = r_value_32;
        }

        let mut r_value_35 = prev_values[1];
        if let Ok(val) = read_stable_adc(&mut adc1, &mut pin_35) {
            r_value_35 = val;
            prev_values[1] = r_value_35;
        }
        
        let _ = uart.write(&packet::build(&[r_value_35, r_value_32]));*/
    }
}

struct ReadableInput<'a, ADCI, PIN> {
    adc: &'a Adc<'a, ADCI, Blocking>,
    pin: AdcPin<PIN, ADCI>,
}

trait Readable {
    fn read(&mut self) -> nb::Result<u16, ()>;
}

impl<'a, ADCI, PIN> Readable for ReadableInput<'a, ADCI, PIN>
where
    PIN: AdcChannel, ADCI: esp_hal::analog::adc::RegisterAccess
{
    fn read(&mut self) -> nb::Result<u16, ()> {
        let mut sum: u32 = 0;
        let samples = 256;

        for _ in 0..samples {
            let value = nb::block!(self.adc.read_oneshot(&mut self.pin))? as u32;
            sum += value;
        }

        let average = (sum / samples) as u16;
        Ok(average / 2)
    }
}


fn initialize_pins() {

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