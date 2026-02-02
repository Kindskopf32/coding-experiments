# encode-rs

A Rust-based video encoding job queue system using PostgreSQL and FFmpeg. Efficiently manage and process video transcoding jobs with a scalable worker architecture.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Adding Jobs](#adding-jobs)
  - [Running Workers](#running-workers)
  - [Monitoring](#monitoring)
- [Development](#development)
  - [Building](#building)
  - [Testing](#testing)
  - [Code Quality](#code-quality)
- [Architecture](#architecture)
- [Deployment](#deployment)
- [Troubleshooting](#troubleshooting)

## Features

- **Job Queue Management**: Persistent job queue backed by PostgreSQL with automatic migrations
- **Worker Architecture**: Asynchronous workers that process jobs from the queue
- **FFmpeg Integration**: Direct integration with FFmpeg for video transcoding
- **Configurable**: TOML-based configuration for database, worker, and optional FFmpeg settings
- **Flexible Encoding Options**: Support for custom codecs, presets, and quality settings
- **Monitoring**: Structured logging with `tracing` for observability
- **Error Handling**: Robust error handling with job status tracking
- **Race Condition Safety**: Uses `SELECT FOR UPDATE SKIP LOCKED` for safe concurrent job claiming
- **Progress Display**: Optional real-time encoding progress output

## Prerequisites

Before installing, ensure you have the following installed:

- **Rust** (Edition 2024 or later) - [Install Rust](https://rustup.rs/)
- **PostgreSQL** (12 or later) - [Install PostgreSQL](https://www.postgresql.org/download/)
- **FFmpeg** - [Install FFmpeg](https://ffmpeg.org/download.html)

Verify your installations:

```bash
rustc --version          # Should show 1.85.0 or later
psql --version           # Should show 12.x or later
ffmpeg -version          # Should show FFmpeg version
```

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd encode-rs
```

### 2. Set Up PostgreSQL

Create a database for the encoding queue:

```bash
# Create database
createdb encode

# Verify connection
psql encode -c "SELECT version();"
```

### 3. Configure the Application

Copy and customize the configuration file:

```bash
cp config.toml config.toml.local
```

Edit `config.toml.local` with your settings:

```toml
[database]
url = "postgresql://username:password@localhost/encode"

[worker]
poll_interval_seconds = 5
# workdir = "/media/encode"  # Optional: Set working directory for workers
```

### 4. Build and Run

```bash
# Development build
cargo build

# Production build (optimized)
cargo build --release

# The binary will be available at:
# - Development: target/debug/encode
# - Production: target/release/encode
```

## Configuration

### Database

```toml
[database]
url = "postgresql://username:password@host:port/database"
```

Supports standard PostgreSQL connection strings. For production, consider using environment variables or a secrets manager.

### Worker

```toml
[worker]
poll_interval_seconds = 5  # Seconds between job polls
workdir = "/media/encode"  # Optional: Working directory for encoding
```

**Options:**
- `poll_interval_seconds`: How often workers check for new jobs (default: 5)
- `workdir`: Optional base directory for all file operations

### FFmpeg (Optional)

FFmpeg settings can be configured globally or per-job:

```toml
[ffmpeg]
path = "ffmpeg"           # Path to FFmpeg binary
```

Common codecs and their use cases:

| Codec | Format | Best For |
|-------|--------|----------|
| `libx264` | H.264/AVC | Maximum compatibility |
| `libx265` | H.265/HEVC | Better compression, newer devices |
| `libsvtav1` | AV1 | Modern, best compression (slower) |
| `libvpx-vp9` | VP9 | Web streaming |

Presets (speed vs. quality trade-off):

| Preset | Speed | Quality |
|--------|-------|---------|
| `ultrafast` | Fastest | Lowest |
| `superfast` | Very Fast | Low |
| `veryfast` | Fast | Fair |
| `faster` | Quick | Good |
| `fast` | Moderate | Better |
| `medium` | Balanced | Balanced (default) |
| `slow` | Slow | Good |
| `slower` | Slower | Better |
| `veryslow` | Slowest | Best |

CRF (Constant Rate Factor) values:
- `0-17`: Visually lossless (large files)
- `18-23`: High quality (default: 23)
- `24-28`: Good quality, smaller files
- `29-51`: Lower quality, smallest files

## Usage

### Adding Jobs

Add video encoding jobs to the queue:

```bash
# Basic usage with defaults (libx264, medium preset, crf 23)
encode add input.mp4 output.mp4

# Using cargo run
cargo run -- add input.mp4 output.mp4

# With custom codec
cargo run -- add input.mp4 output.mp4 --codec libx265

# With custom preset and quality
cargo run -- add input.mp4 output.mp4 --preset slow --crf 20

# Complete example
cargo run -- add /path/to/input.mkv /path/to/output.mp4 --codec libx264 --preset slow --crf 18
```

**Command Options:**

```
encode add [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input video file path
  <OUTPUT>  Output video file path

Options:
      --codec <CODEC>    Video codec (default: libx264)
      --preset <PRESET>  Encoding preset (default: medium)
      --crf <CRF>        CRF quality value 0-51, lower is better (default: 23)
  -h, --help             Print help
```

### Running Workers

Start workers to process jobs from the queue:

```bash
# Continuous mode (default) - keeps running and polling for jobs
encode worker

# Using cargo run
cargo run -- worker

# Burst mode - exits when no more jobs available
encode worker --burst

# With progress display
encode worker --progress

# Burst mode with progress
encode worker --burst --progress
```

**Command Options:**

```
encode worker [OPTIONS]

Options:
  -b, --burst     Exit when no more jobs (burst mode)
      --progress  Show encoding progress output
  -h, --help      Print help
```

**Running Multiple Workers:**

You can run multiple workers simultaneously for parallel processing:

```bash
# Terminal 1
cargo run -- worker

# Terminal 2
cargo run -- worker

# Terminal 3 (burst mode for batch processing)
cargo run -- worker --burst
```

### Monitoring

#### View Job Status

```bash
# List all jobs
psql encode -c "SELECT id, status, input_path, output_path, created_at FROM jobs;"

# Count jobs by status
psql encode -c "SELECT status, COUNT(*) FROM jobs GROUP BY status;"

# View failed jobs
psql encode -c "SELECT id, input_path, error_message FROM jobs WHERE status = 'failed';"
```

#### Logging

Set the log level using the `RUST_LOG` environment variable:

```bash
# Error only
RUST_LOG=error cargo run -- worker

# Info level (default)
RUST_LOG=info cargo run -- worker

# Debug level (verbose)
RUST_LOG=debug cargo run -- worker

# Trace level (very verbose)
RUST_LOG=trace cargo run -- worker
```

## Development

### Building

```bash
# Development build (with debug symbols)
cargo build

# Release build (optimized)
cargo build --release

# Check compilation without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test cli::
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run Clippy lints
cargo clippy

# Run Clippy with all features and tests
cargo clippy --all-targets --all-features

# Fix auto-fixable Clippy warnings
cargo clippy --fix
```

## Architecture

### Database Schema

The system uses PostgreSQL with automatic migrations:

**Migrations:**
- `migrations/001_initial.sql` - Initial schema creation
- `migrations/002_add_done_jobs_index.sql` - Performance optimization index

**Jobs Table:**

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Primary key, auto-generated |
| `status` | VARCHAR(20) | Job status: `pending`, `processing`, `done`, `failed` |
| `input_path` | TEXT | Path to input video file |
| `output_path` | TEXT | Path for output video file |
| `video_codec` | TEXT | FFmpeg video codec (e.g., libx264) |
| `preset` | TEXT | Encoding preset (e.g., medium, slow) |
| `crf` | INTEGER | Quality value (0-51, lower is better) |
| `error_message` | TEXT | Error details if job failed |
| `created_at` | TIMESTAMPTZ | Job creation timestamp |
| `started_at` | TIMESTAMPTZ | Job start timestamp |
| `completed_at` | TIMESTAMPTZ | Job completion timestamp |

### Project Structure

```
encode-rs/
├── src/
│   ├── main.rs      # Application entry point
│   ├── cli.rs       # CLI argument parsing with clap
│   ├── config.rs    # TOML configuration management
│   ├── db.rs        # PostgreSQL database operations
│   ├── ffmpeg.rs    # FFmpeg integration and validation
│   └── worker.rs    # Worker process logic
├── migrations/
│   ├── 001_initial.sql      # Initial database schema
│   └── 002_add_done_jobs_index.sql  # Performance index
├── config.toml      # Application configuration
├── Cargo.toml       # Rust dependencies
└── README.md        # This file
```

### Module Overview

#### `cli.rs`
Command-line interface using `clap`. Supports:
- `add` command: Add encoding jobs with codec, preset, and CRF options
- `worker` command: Run workers in continuous or burst mode with optional progress

#### `config.rs`
TOML-based configuration loading with support for:
- Database connection settings
- Worker polling intervals and working directory
- Optional FFmpeg binary path

#### `db.rs`
Database abstraction layer with:
- Connection pooling via `sqlx`
- Automatic migration execution
- Job CRUD operations
- Safe concurrent job claiming with `FOR UPDATE SKIP LOCKED`

#### `ffmpeg.rs`
FFmpeg integration:
- Input file validation
- Output directory creation
- FFmpeg execution with configurable parameters
- Error handling and structured logging

#### `worker.rs`
Worker process logic:
- Database polling for pending jobs
- Job processing with FFmpeg
- Status updates and error tracking
- Support for continuous and burst modes

## Deployment

### Docker

Example multi-stage Dockerfile:

```dockerfile
# Build stage
FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ffmpeg \
    postgresql-client \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 encode

# Copy binary and config
COPY --from=builder /app/target/release/encode /usr/local/bin/
COPY --from=builder /app/config.toml /etc/encode/

USER encode
WORKDIR /tmp

CMD ["encode", "worker"]
```

Build and run:

```bash
# Build image
docker build -t encode-rs .

# Run worker
docker run -v /path/to/videos:/videos encode-rs
```

### Systemd Service

Create `/etc/systemd/system/encode-worker.service`:

```ini
[Unit]
Description=Video Encoding Worker
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=encode
Group=encode
WorkingDirectory=/opt/encode
ExecStart=/opt/encode/encode worker
Restart=always
RestartSec=10
Environment=RUST_LOG=info
Environment=PGSSLMODE=disable

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/encode/output

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable encode-worker
sudo systemctl start encode-worker

# Check status
sudo systemctl status encode-worker
sudo journalctl -u encode-worker -f
```

## Troubleshooting

### FFmpeg Not Found

**Problem:** `Error: Failed to execute FFmpeg: No such file or directory`

**Solutions:**
1. Install FFmpeg: `sudo apt-get install ffmpeg` (Debian/Ubuntu) or `brew install ffmpeg` (macOS)
2. Verify in PATH: `which ffmpeg`
3. Specify full path in config:
   ```toml
   [ffmpeg]
   path = "/usr/bin/ffmpeg"
   ```

### Database Connection Errors

**Problem:** `Error: Failed to connect to database`

**Solutions:**
1. Verify PostgreSQL is running: `sudo systemctl status postgresql`
2. Check connection URL in `config.toml`
3. Verify database exists: `psql -l | grep encode`
4. Test connection manually: `psql encode -c "SELECT 1;"`
5. Check user permissions:
   ```sql
   GRANT ALL PRIVILEGES ON DATABASE encode TO username;
   ```

### Jobs Stuck in "processing"

If a worker crashes or is killed, jobs may remain stuck:

```bash
# Reset stuck jobs back to pending
psql encode -c "UPDATE jobs SET status = 'pending', started_at = NULL WHERE status = 'processing';"
```

### Build Errors

**Problem:** Compilation fails

**Solutions:**
1. Update Rust: `rustup update`
2. Clean build: `cargo clean && cargo build`
3. Check dependencies: `cargo tree`

### Performance Issues

**High CPU usage:**
- Use faster preset: `--preset fast` or `--preset veryfast`
- Use hardware acceleration if available (NVENC, QSV, VAAPI)

**Slow job pickup:**
- Reduce `poll_interval_seconds` in config
- Run multiple workers
- Check for database connection pool limits

**Large queue backlog:**
```bash
# Monitor queue depth
watch -n 5 'psql encode -c "SELECT status, COUNT(*) FROM jobs GROUP BY status;"'
```

## License

MIT License - See LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run linting: `cargo clippy && cargo fmt -- --check`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## Future Enhancements

- [ ] Job progress tracking with percentage complete
- [ ] Web dashboard for monitoring and management
- [ ] Retry logic with exponential backoff
- [ ] Priority queues for urgent jobs
- [ ] Multiple output formats per job
- [ ] Hardware acceleration support (NVENC, QSV, VAAPI)
- [ ] REST API for external integrations
- [ ] Job templates for common encoding profiles
- [ ] Email notifications on job completion/failure
- [ ] Statistics and reporting dashboard
