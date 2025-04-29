use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::{unbounded, Receiver};
use log::{debug, error, info};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use notify::event::{DataChange, ModifyKind};
use serde::Deserialize;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize)]
pub struct Config {
    pub mappings: Vec<Mapping>,
    pub serial: String,
    pub baud_rate: u32,
    pub timeout: u64,
    pub quality: Quality
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    #[default]
    Default,
    High
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Mapping {
    Master { pin: u32, inverted: bool, master: u32 },
    Device { pin: u32, inverted: bool, device: String },
    App { pin: u32, inverted: bool, app: String },
    Midi { pin: u32, inverted: bool, midi: u32 },
    Unmapped { pin: u32, inverted: bool },
}

pub fn get_config_path() -> std::io::Result<PathBuf> {
    let base = xdg::BaseDirectories::with_prefix("deejx")?;
    base.place_config_file("profile.deejx.yml")
}

pub fn read_config(path: &PathBuf) -> anyhow::Result<Config> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        info!("Creating missing profile...");
        fs::write(&path, include_str!("../example.yml"))?;
    }

    let raw = fs::read_to_string(path)?;
    let config = serde_yml::from_str(raw.as_str())?;
    debug!("Read config, done.");
    Ok(config)
}

impl Config {
    pub(crate) fn update(&mut self, config_path: &PathBuf, watcher: &Receiver<Event>) {
        let mut is_ok = true;
        loop {
            while let Ok(event) = watcher.try_recv() {
                if event.kind == EventKind::Modify(ModifyKind::Data(DataChange::Any)) {
                    info!("Reloading config");
                    match read_config(&config_path) {
                        Ok(new) => {
                            *self = new;
                            is_ok = true;
                        },
                        Err(err) => {
                            log::error!("Config invalid: {}", err);
                            is_ok = false;
                        }
                    }
                }
            }
            if is_ok {
                break;
            }
            sleep(Duration::from_millis(100));
        }
    }
}

pub fn prepare_config(path: &PathBuf) -> anyhow::Result<(Config, Receiver<Event>)> {
    let mut config = read_config(path);
    let config_events = start_config_watcher(path);
    if config.is_err() {
        error!("{}", config.as_ref().unwrap_err());
    }
    while config.is_err() {
        sleep(Duration::from_millis(100));
        
        // Poll for config file changes
        while let Ok(event) = config_events.try_recv() {
            if event.kind == EventKind::Modify(ModifyKind::Data(DataChange::Any)) {
                config = read_config(path);
                error!("{}", config.as_ref().unwrap_err());
            }
        }
    };
    Ok((config?, config_events))
}

fn start_config_watcher<P: AsRef<Path>>(path: P) -> Receiver<Event> {
    let (tx, rx) = unbounded();

    // Spawn a thread for the watcher
    let path = path.as_ref().to_owned();
    thread::spawn(move || {
        // Create watcher, automatically debounces events
        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    tx.send(event).expect("Failed to send event");
                }
                Err(e) => {
                    error!("Watch error: {:?}", e);
                }
            }
        }).expect("Failed to create watcher");

        watcher
            .watch(&path, RecursiveMode::NonRecursive)
            .expect("Failed to start watching");

        // Keep the watcher alive
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

    rx
}