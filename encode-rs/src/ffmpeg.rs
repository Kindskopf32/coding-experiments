use anyhow::{Context, Result};
use std::path::Path;
use tokio::process::Command;
use tracing::{debug, error, info};

use crate::db::Job;

pub struct FfmpegRunner {
    ffmpeg_path: String,
}

impl FfmpegRunner {
    pub fn new(ffmpeg_path: impl Into<String>) -> Self {
        Self {
            ffmpeg_path: ffmpeg_path.into(),
        }
    }

    pub async fn transcode(&self, job: &Job) -> Result<()> {
        info!(
            "Starting transcoding job {}: {} -> {}",
            job.id, job.input_path, job.output_path
        );

        // Validate input file exists
        if !Path::new(&job.input_path).exists() {
            return Err(anyhow::anyhow!(
                "Input file not found: {}",
                job.input_path
            ));
        }

        // Ensure output directory exists
        if let Some(parent) = Path::new(&job.output_path).parent() {
            tokio::fs::create_dir_all(parent).await.with_context(|| {
                format!("Failed to create output directory: {}", parent.display())
            })?;
        }

        debug!(
            "FFmpeg args: -i {} -c:v {} -preset {} -crf {} -y {}",
            job.input_path, job.video_codec, job.preset, job.crf, job.output_path
        );

        let output = Command::new(&self.ffmpeg_path)
            .arg("-i")
            .arg(&job.input_path)
            .arg("-c:v")
            .arg(&job.video_codec)
            .arg("-preset")
            .arg(&job.preset)
            .arg("-crf")
            .arg(job.crf.to_string())
            .arg("-y") // Overwrite output file
            .arg(&job.output_path)
            .output()
            .await
            .with_context(|| format!("Failed to execute FFmpeg: {}", self.ffmpeg_path))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg failed for job {}: {}", job.id, stderr);
            return Err(anyhow::anyhow!("FFmpeg failed: {}", stderr));
        }

        info!("Job {} completed successfully", job.id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_job(input: &str, output: &str) -> Job {
        Job {
            id: Uuid::new_v4(),
            status: "pending".to_string(),
            input_path: input.to_string(),
            output_path: output.to_string(),
            video_codec: "libx264".to_string(),
            preset: "medium".to_string(),
            crf: 23,
            error_message: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }

    #[test]
    fn test_ffmpeg_runner_new() {
        let runner = FfmpegRunner::new("/usr/bin/ffmpeg");
        assert_eq!(runner.ffmpeg_path, "/usr/bin/ffmpeg");
    }

    #[test]
    fn test_ffmpeg_runner_new_from_string() {
        let path = String::from("/opt/ffmpeg");
        let runner = FfmpegRunner::new(path);
        assert_eq!(runner.ffmpeg_path, "/opt/ffmpeg");
    }

    #[tokio::test]
    async fn test_transcode_missing_input_file() {
        let runner = FfmpegRunner::new("ffmpeg");
        let job = create_test_job("/nonexistent/path/input.mp4", "/output.mp4");

        let result = runner.transcode(&job).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Input file not found"));
    }

    #[test]
    fn test_job_structure() {
        let job = create_test_job("/input.mp4", "/output.mp4");
        
        assert_eq!(job.input_path, "/input.mp4");
        assert_eq!(job.output_path, "/output.mp4");
        assert_eq!(job.video_codec, "libx264");
        assert_eq!(job.preset, "medium");
        assert_eq!(job.crf, 23);
        assert!(job.error_message.is_none());
    }

    #[test]
    fn test_job_with_custom_settings() {
        let mut job = create_test_job("/input.mp4", "/output.mp4");
        job.video_codec = "libx265".to_string();
        job.preset = "slow".to_string();
        job.crf = 18;

        assert_eq!(job.video_codec, "libx265");
        assert_eq!(job.preset, "slow");
        assert_eq!(job.crf, 18);
    }
}
