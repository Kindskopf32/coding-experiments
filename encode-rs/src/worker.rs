use anyhow::Result;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};

use crate::config::Config;
use crate::db::Database;
use crate::ffmpeg::FfmpegRunner;

pub async fn run_worker(config: Config, burst_mode: bool, show_progress: bool) -> Result<()> {
    info!(
        "Starting worker (burst mode: {}, progress: {})",
        burst_mode, show_progress
    );

    // Validate ffmpeg and ffprobe are available before starting
    FfmpegRunner::validate_binaries().await?;

    let db = Database::connect(&config.database.url).await?;
    let ffmpeg = FfmpegRunner::new();

    // Cleanup old done jobs at startup
    let deleted = db
        .cleanup_old_done_jobs(config.worker.cleanup_done_jobs_after as i64)
        .await?;
    if deleted > 0 {
        info!("Cleaned up {} old done job(s) at startup", deleted);
    }

    loop {
        // Check if we should exit in burst mode
        if burst_mode {
            let has_jobs = db.has_pending_jobs().await?;
            if !has_jobs {
                info!("No more pending jobs, exiting burst mode");
                break;
            }
        }

        // Try to claim a job
        match db.claim_next_job().await? {
            Some(job) => {
                // Cleanup old done jobs after claiming a new job
                let deleted = db
                    .cleanup_old_done_jobs(config.worker.cleanup_done_jobs_after as i64)
                    .await?;
                if deleted > 0 {
                    info!("Cleaned up {} old done job(s)", deleted);
                }

                // Resolve paths for logging
                let workdir = config.worker.workdir.as_deref();
                let input_display = if let Some(workdir) = workdir {
                    if Path::new(&job.input_path).is_absolute() {
                        &job.input_path
                    } else {
                        &format!("{}/{}", workdir.display(), job.input_path)
                    }
                } else {
                    &job.input_path
                };
                let output_display = if let Some(workdir) = workdir {
                    if Path::new(&job.output_path).is_absolute() {
                        &job.output_path
                    } else {
                        &format!("{}/{}", workdir.display(), job.output_path)
                    }
                } else {
                    &job.output_path
                };

                info!(
                    "Processing job {}: {} -> {} (codec: {}, preset: {}, crf: {})",
                    job.id, input_display, output_display, job.video_codec, job.preset, job.crf
                );

                // Process the job
                match ffmpeg
                    .transcode(
                        &job,
                        show_progress,
                        workdir,
                        config.worker.tmp_dir.as_deref(),
                    )
                    .await
                {
                    Ok(_) => {
                        db.mark_done(job.id).await?;
                        info!("Job {} marked as done", job.id);
                    }
                    Err(e) => {
                        let error_msg = format!("{}", e);
                        warn!("Job {} failed: {}", job.id, error_msg);
                        db.mark_failed(job.id, &error_msg).await?;
                    }
                }
            }
            None => {
                if burst_mode {
                    // No jobs available, exit burst mode
                    info!("No jobs available, exiting burst mode");
                    break;
                } else {
                    // Normal mode: wait and poll again
                    debug!(
                        "No pending jobs, waiting {} seconds...",
                        config.worker.poll_interval
                    );
                    sleep(Duration::from_secs(config.worker.poll_interval)).await;
                }
            }
        }
    }

    info!("Worker shutting down");
    Ok(())
}
