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
