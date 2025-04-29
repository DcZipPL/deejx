#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub fn get_controller() -> Box<impl AudioControl> {
    unix::new()
}

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub fn get_controller() -> Box<impl AudioControl> {
    windows::new()
}

pub const RAW_MAX: f32 = 2047.;
pub const MINIMUM: f32 = 0.;
pub const MAXIMUM: f32 = 100.;

pub trait AudioControl {
    fn name(&self) -> &'static str;
    fn set_master_volume(&mut self, device_index: u32, volume: u16);
    fn set_app_volume_by_index(&mut self, app_index: u32, volume: u16);
    fn set_app_volume_by_name(&mut self, name: &str, volume: u16);
}