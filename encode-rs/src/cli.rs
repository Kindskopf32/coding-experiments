use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "encode-rs")]
#[command(about = "Video encoding job queue")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a job to the queue
    Add {
        /// Input video file path
        input: PathBuf,
        /// Output video file path
        output: PathBuf,
        /// Video codec (default: libx264)
        #[arg(long)]
        codec: Option<String>,
        /// Encoding preset (default: medium)
        #[arg(long)]
        preset: Option<String>,
        /// CRF quality value 0-51, lower is better (default: 23)
        #[arg(long)]
        crf: Option<i32>,
    },
    /// Run worker to process jobs
    Worker {
        /// Exit when no more jobs (burst mode)
        #[arg(short, long)]
        burst: bool,
        /// Show encoding progress
        #[arg(long)]
        progress: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_cli_add_command() {
        let args = vec!["encode", "add", "/input.mp4", "/output.mp4"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Add {
                input,
                output,
                codec,
                preset,
                crf,
            } => {
                assert_eq!(input, PathBuf::from("/input.mp4"));
                assert_eq!(output, PathBuf::from("/output.mp4"));
                assert!(codec.is_none());
                assert!(preset.is_none());
                assert!(crf.is_none());
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_cli_add_with_options() {
        let args = vec![
            "encode",
            "add",
            "/input.mp4",
            "/output.mp4",
            "--codec",
            "libx265",
            "--preset",
            "slow",
            "--crf",
            "18",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Add {
                input,
                output,
                codec,
                preset,
                crf,
            } => {
                assert_eq!(input, PathBuf::from("/input.mp4"));
                assert_eq!(output, PathBuf::from("/output.mp4"));
                assert_eq!(codec, Some("libx265".to_string()));
                assert_eq!(preset, Some("slow".to_string()));
                assert_eq!(crf, Some(18));
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_cli_worker_command() {
        let args = vec!["encode", "worker"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Worker { burst, progress } => {
                assert!(!burst);
                assert!(!progress);
            }
            _ => panic!("Expected Worker command"),
        }
    }

    #[test]
    fn test_cli_worker_burst_mode() {
        let args = vec!["encode", "worker", "--burst"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Worker { burst, progress } => {
                assert!(burst);
                assert!(!progress);
            }
            _ => panic!("Expected Worker command"),
        }
    }

    #[test]
    fn test_cli_worker_burst_short_flag() {
        let args = vec!["encode", "worker", "-b"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Worker { burst, progress } => {
                assert!(burst);
                assert!(!progress);
            }
            _ => panic!("Expected Worker command"),
        }
    }

    #[test]
    fn test_cli_worker_progress_flag() {
        let args = vec!["encode", "worker", "--progress"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Worker { burst, progress } => {
                assert!(!burst);
                assert!(progress);
            }
            _ => panic!("Expected Worker command"),
        }
    }

    #[test]
    fn test_cli_worker_burst_and_progress() {
        let args = vec!["encode", "worker", "--burst", "--progress"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Worker { burst, progress } => {
                assert!(burst);
                assert!(progress);
            }
            _ => panic!("Expected Worker command"),
        }
    }
}
