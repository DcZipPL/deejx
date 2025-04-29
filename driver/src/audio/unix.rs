use libpulse_binding::volume::{ChannelVolumes, Volume};
use pulsectl::controllers::{AppControl, DeviceControl, SinkController};
use pulsectl::controllers::types::ApplicationInfo;
use crate::audio::{AudioControl, MAXIMUM, MINIMUM, RAW_MAX};

pub(super) fn new() -> Box<impl AudioControl> {
    let controller = SinkController::create().unwrap();
    Box::new(UnixController {
        controller
    })
}

pub(super) struct UnixController {
    controller: SinkController
}

impl AudioControl for UnixController {
    fn name(&self) -> &'static str {
        "libpulse"
    }

    fn set_master_volume(&mut self, device_index: u32, volume: u16) {
        let volume = ((volume as f32) / RAW_MAX).max(MINIMUM).min(MAXIMUM);
        let volume: u32 = ((volume) * 65536.0).round() as u32;
        
        self.controller.set_device_volume_by_index(device_index, ChannelVolumes::default().set(2, Volume(volume)))
    }

    fn set_app_volume_by_index(&mut self, app_index: u32, volume: u16) {
        let app = self.controller.get_app_by_index(app_index);
        match app {
            Ok(app) => {
                set_app_volume(&mut self.controller, app, volume);
            }
            Err(err) => {
                println!("Could not set application volume. {:?}", err);
            }
        }
    }

    fn set_app_volume_by_name(&mut self, name: &str, volume: u16) {
        if let Ok(apps) = self.controller.list_applications() {
            for app in apps {
                if app.name.clone().is_some_and(|s| s.to_lowercase().contains(name)) {
                    set_app_volume(&mut self.controller, app, volume);
                }
            }
        }
    }
}

fn set_app_volume(controller: &mut SinkController, app: ApplicationInfo, volume: u16) {
    let app_volume = app.volume.avg().0 as f32 / 65536.;
    let new_volume: f32 = (volume as f32 / RAW_MAX * 100.0).round() / 100.0;

    let volume_change = new_volume - app_volume;
    // println!("left: {:?}, right: {:?}, change: {}", app.volume.get()[0], app.volume.get()[1], volume_change);
    if volume_change > 0. {
        controller.increase_app_volume_by_percent(app.index, volume_change.abs() as f64);
    } else {
        controller.decrease_app_volume_by_percent(app.index, volume_change.abs() as f64);
    }
}