#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

trait AudioControl {
    fn set_master_volume(&self, volume: f32);
    fn set_app_volume(&self, volume: f32);
}