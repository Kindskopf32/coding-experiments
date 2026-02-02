use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::interval;
use tracing::{debug, error, info};

use crate::db::Job;

const FFMPEG_CMD: &str = "ffmpeg";
const FFPROBE_CMD: &str = "ffprobe";

pub struct FfmpegRunner;

impl FfmpegRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn validate_binaries() -> Result<()> {
        Self::check_binary(FFMPEG_CMD).await?;
        Self::check_binary(FFPROBE_CMD).await?;
        Ok(())
    }

    async fn check_binary(binary: &str) -> Result<()> {
        let output = Command::new("which")
            .arg(binary)
            .output()
            .await
            .with_context(|| format!("Failed to check for {} binary", binary))?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "{} not found in PATH. Please ensure {} is installed and available.",
                binary,
                binary
            ));
        }

        Ok(())
    }

    /// Resolves a path against the workdir if one is configured.
    /// If the path is absolute or no workdir is set, returns the path as-is.
    fn resolve_path(path: &str, workdir: Option<&Path>) -> PathBuf {
        if let Some(workdir) = workdir {
            let path_buf = PathBuf::from(path);
            if path_buf.is_absolute() {
                path_buf
            } else {
                workdir.join(path)
            }
        } else {
            PathBuf::from(path)
        }
    }

    pub async fn transcode(
        &self,
        job: &Job,
        show_progress: bool,
        workdir: Option<&Path>,
        tmp_dir: Option<&Path>,
    ) -> Result<()> {
        // Resolve input/output paths against workdir if provided
        let input_path = Self::resolve_path(&job.input_path, workdir);
        let output_path = Self::resolve_path(&job.output_path, workdir);

        info!(
            "Starting transcoding job {}: {} -> {}",
            job.id,
            input_path.display(),
            output_path.display()
        );

        // Validate input file exists
        if !input_path.exists() {
            return Err(anyhow::anyhow!(
                "Input file not found: {}",
                input_path.display()
            ));
        }

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await.with_context(|| {
                format!("Failed to create output directory: {}", parent.display())
            })?;
        }

        // Determine encoding target and whether we're using temp storage
        let (encoding_target, using_temp) = if let Some(tmp_dir) = tmp_dir {
            // Create tmp_dir if needed
            fs::create_dir_all(tmp_dir).await.with_context(|| {
                format!("Failed to create temp directory: {}", tmp_dir.display())
            })?;

            // Generate temp file path with job ID and proper extension
            let extension = output_path.extension().map(|e| e.to_string_lossy());
            let temp_filename = if let Some(ext) = extension {
                format!("encode_{}.{}", job.id, ext)
            } else {
                format!("encode_{}", job.id)
            };
            let temp_path = tmp_dir.join(&temp_filename);

            info!(
                "Using temp storage: encoding to {} first, then copying to {}",
                temp_path.display(),
                output_path.display()
            );

            (temp_path, true)
        } else {
            (output_path.clone(), false)
        };

        // Get video duration for progress calculation
        let duration_ms = self.get_video_duration(&input_path.to_string_lossy()).await;

        debug!(
            "FFmpeg args: -i {} -c:v {} -preset {} -crf {} -c:a libopus -b:a 96k -progress pipe:1 -y {}",
            input_path.display(),
            job.video_codec,
            job.preset,
            job.crf,
            encoding_target.display()
        );

        // Spawn FFmpeg with progress output
        let mut child = Command::new(FFMPEG_CMD)
            .arg("-i")
            .arg(&input_path)
            .arg("-c:v")
            .arg(&job.video_codec)
            .arg("-preset")
            .arg(&job.preset)
            .arg("-crf")
            .arg(job.crf.to_string())
            .arg("-c:a")
            .arg("libopus")
            .arg("-b:a")
            .arg("96k")
            .arg("-progress")
            .arg("pipe:1")
            .arg("-y")
            .arg(&encoding_target)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn FFmpeg: {}", FFMPEG_CMD))?;

        let stdout = child
            .stdout
            .take()
            .context("Failed to capture FFmpeg stdout")?;

        // Shared state for progress tracking
        let current_time_ms = Arc::new(AtomicU64::new(0));
        let current_time_ms_reader = Arc::clone(&current_time_ms);

        // Task to read FFmpeg progress output
        let reader_task = tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                if let Some(time_str) = line.strip_prefix("out_time_ms=")
                    && let Ok(time_us) = time_str.parse::<u64>()
                {
                    // FFmpeg outputs time in microseconds, convert to milliseconds
                    current_time_ms_reader.store(time_us / 1000, Ordering::Relaxed);
                }
            }
        });

        // Clone job ID for use in spawned task
        let job_id = job.id;

        // Task to log progress every 5 seconds (only if progress flag is enabled)
        let progress_logger_task = if show_progress {
            Some(tokio::spawn(async move {
                let mut ticker = interval(Duration::from_secs(5));

                loop {
                    ticker.tick().await;

                    let time_ms = current_time_ms.load(Ordering::Relaxed);

                    if time_ms == 0 {
                        info!("Job {} encoding: starting...", job_id);
                        continue;
                    }

                    let elapsed = format_duration(time_ms);

                    if let Some(total_ms) = duration_ms {
                        let percentage = (time_ms as f64 / total_ms as f64) * 100.0;
                        let total = format_duration(total_ms);
                        info!(
                            "Job {} encoding: {:.1}% complete ({} / {})",
                            job_id, percentage, elapsed, total
                        );
                    } else {
                        info!("Job {} encoding: {} elapsed", job_id, elapsed);
                    }
                }
            }))
        } else {
            None
        };

        // Wait for FFmpeg to complete
        let exit_status = child
            .wait()
            .await
            .with_context(|| "Failed to wait for FFmpeg process")?;

        // Stop the progress logger if it was started
        if let Some(task) = progress_logger_task {
            task.abort();
            let _ = task.await;
        }

        // Stop the reader task
        reader_task.abort();
        let _ = reader_task.await;

        if !exit_status.success() {
            // Try to capture stderr for error message
            let mut stderr_output = String::new();
            if let Some(stderr) = child.stderr.take() {
                let mut reader = BufReader::new(stderr);
                use tokio::io::AsyncReadExt;
                let mut buf = Vec::new();
                if reader.read_to_end(&mut buf).await.is_ok() {
                    stderr_output = String::from_utf8_lossy(&buf).to_string();
                }
            }

            if stderr_output.is_empty() {
                stderr_output = "FFmpeg exited with non-zero status".to_string();
            }

            error!("FFmpeg failed for job {}: {}", job.id, stderr_output);

            // If using temp storage, keep the temp file for debugging
            if using_temp {
                info!(
                    "Job {}: Temp file kept for debugging at: {}",
                    job.id,
                    encoding_target.display()
                );
            }

            return Err(anyhow::anyhow!("FFmpeg failed: {}", stderr_output));
        }

        // If using temp storage, copy to final output and clean up
        if using_temp {
            info!(
                "Job {}: Copying from {} to {}",
                job.id,
                encoding_target.display(),
                output_path.display()
            );

            fs::copy(&encoding_target, &output_path)
                .await
                .with_context(|| {
                    format!(
                        "Failed to copy file from {} to {}",
                        encoding_target.display(),
                        output_path.display()
                    )
                })?;

            info!(
                "Job {}: Removing temp file {}",
                job.id,
                encoding_target.display()
            );

            fs::remove_file(&encoding_target).await.with_context(|| {
                format!("Failed to remove temp file: {}", encoding_target.display())
            })?;
        }

        info!("Job {} completed successfully", job.id);
        Ok(())
    }

    async fn get_video_duration(&self, input_path: &str) -> Option<u64> {
        let output = Command::new(FFPROBE_CMD)
            .arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(input_path)
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                let duration_str = String::from_utf8_lossy(&output.stdout);
                duration_str
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .map(|seconds| (seconds * 1000.0) as u64)
            }
            _ => {
                debug!("Failed to get video duration for {}", input_path);
                None
            }
        }
    }
}

fn format_duration(milliseconds: u64) -> String {
    let total_seconds = milliseconds / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
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
        let _runner = FfmpegRunner::new();
        // FfmpegRunner now uses hardcoded commands from PATH
    }

    #[tokio::test]
    async fn test_transcode_missing_input_file() {
        let runner = FfmpegRunner::new();
        let job = create_test_job("/nonexistent/path/input.mp4", "/output.mp4");

        let result = runner.transcode(&job, false, None, None).await;
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

    #[test]
    fn test_format_duration_seconds_only() {
        assert_eq!(format_duration(45000), "00:45"); // 45 seconds
        assert_eq!(format_duration(90000), "01:30"); // 1 minute 30 seconds
    }

    #[test]
    fn test_format_duration_with_hours() {
        assert_eq!(format_duration(3661000), "01:01:01"); // 1 hour 1 minute 1 second
        assert_eq!(format_duration(7200000), "02:00:00"); // 2 hours
    }

    #[test]
    fn test_format_duration_zero() {
        assert_eq!(format_duration(0), "00:00");
    }
}
