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
    pub ffprobe_path: String,
    pub default_codec: String,
    pub default_preset: String,
    pub default_crf: i32,
}

#[derive(Debug, Deserialize)]
pub struct WorkerConfig {
    #[serde(rename = "poll_interval_seconds")]
    pub poll_interval: u64,
    #[serde(rename = "cleanup_done_jobs_after_seconds")]
    pub cleanup_done_jobs_after: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Path::new("config.toml");
        let content = fs::read_to_string(config_path)?;
        Self::from_str(&content)
    }

    pub fn from_str(content: &str) -> Result<Self> {
        let config: Config = toml::from_str(content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_str() {
        let toml_content = r#"
[database]
url = "postgres://user:pass@localhost/test_db"

[ffmpeg]
path = "/usr/bin/ffmpeg"
ffprobe_path = "/usr/bin/ffprobe"
default_codec = "libx264"
default_preset = "medium"
default_crf = 23

[worker]
poll_interval_seconds = 5
cleanup_done_jobs_after_seconds = 300
"#;

        let config = Config::from_str(toml_content).unwrap();

        assert_eq!(
            config.database.url,
            "postgres://user:pass@localhost/test_db"
        );
        assert_eq!(config.ffmpeg.path, "/usr/bin/ffmpeg");
        assert_eq!(config.ffmpeg.ffprobe_path, "/usr/bin/ffprobe");
        assert_eq!(config.ffmpeg.default_codec, "libx264");
        assert_eq!(config.ffmpeg.default_preset, "medium");
        assert_eq!(config.ffmpeg.default_crf, 23);
        assert_eq!(config.worker.poll_interval, 5);
        assert_eq!(config.worker.cleanup_done_jobs_after, 300);
    }

    #[test]
    fn test_config_different_values() {
        let toml_content = r#"
[database]
url = "postgres://admin:secret@db.example.com/production"

[ffmpeg]
path = "/opt/ffmpeg/bin/ffmpeg"
ffprobe_path = "/opt/ffmpeg/bin/ffprobe"
default_codec = "libx265"
default_preset = "slow"
default_crf = 18

[worker]
poll_interval_seconds = 30
cleanup_done_jobs_after_seconds = 600
"#;

        let config = Config::from_str(toml_content).unwrap();

        assert_eq!(
            config.database.url,
            "postgres://admin:secret@db.example.com/production"
        );
        assert_eq!(config.ffmpeg.path, "/opt/ffmpeg/bin/ffmpeg");
        assert_eq!(config.ffmpeg.ffprobe_path, "/opt/ffmpeg/bin/ffprobe");
        assert_eq!(config.ffmpeg.default_codec, "libx265");
        assert_eq!(config.ffmpeg.default_preset, "slow");
        assert_eq!(config.ffmpeg.default_crf, 18);
        assert_eq!(config.worker.poll_interval, 30);
        assert_eq!(config.worker.cleanup_done_jobs_after, 600);
    }

    #[test]
    fn test_config_invalid_toml() {
        let invalid_toml = r#"
[database
url = "postgres://localhost/db"
"#;

        let result = Config::from_str(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_missing_section() {
        let incomplete_toml = r#"
[database]
url = "postgres://localhost/db"
"#;

        let result = Config::from_str(incomplete_toml);
        assert!(result.is_err());
    }
}
