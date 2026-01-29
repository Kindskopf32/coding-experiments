use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub ffmpeg: FfmpegConfig,
    pub worker: WorkerConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct FfmpegConfig {
    pub path: String,
    pub default_codec: String,
    pub default_preset: String,
    pub default_crf: i32,
}

#[derive(Debug, Deserialize)]
pub struct WorkerConfig {
    #[serde(rename = "poll_interval_seconds")]
    pub poll_interval: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Path::new("config.toml");
        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
