use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mappings: Vec<Mapping>,
    pub serial: String,
    pub baud_rate: usize,
    pub quality: Quality
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    #[default]
    Default,
    High
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Mapping {
    Master { pin: u32, inverted: bool, master: u32 },
    App { pin: u32, inverted: bool, app: String },
    Midi { pin: u32, inverted: bool, midi: u32 },
    Unmapped { pin: u32, inverted: bool },
}

pub fn read_config() -> anyhow::Result<Config> {
    let base = xdg::BaseDirectories::with_prefix("deejx")?;
    let path = base.place_config_file("profile.deejx.yml")?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        println!("Creating missing profile...");
        fs::write(&path, include_str!("../example.yml"))?;
    }

    let raw = fs::read_to_string(path)?;
    let config = serde_yml::from_str(raw.as_str())?;
    println!("Read config, done.");
    Ok(config)
}