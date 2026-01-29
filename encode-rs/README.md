# encode-rs

A Rust-based video encoding job queue system using PostgreSQL and FFmpeg. Efficiently manage and process video transcoding jobs with a scalable worker architecture.

## Features

- **Job Queue Management**: Persistent job queue backed by PostgreSQL
- **Worker Architecture**: Asynchronous workers that process jobs from the queue
- **FFmpeg Integration**: Direct integration with FFmpeg for video transcoding
- **Configurable**: TOML-based configuration for easy customization
- **Flexible Encoding Options**: Support for custom codecs, presets, and quality settings
- **Monitoring**: Structured logging with `tracing` for observability
- **Error Handling**: Robust error handling with job retry tracking
- **Race Condition Safety**: Uses `SELECT FOR UPDATE SKIP LOCKED` for safe concurrent job claiming

## Quick Start

### Prerequisites

- **Rust** (Edition 2024 or later)
- **PostgreSQL** (12 or later)
- **FFmpeg** installed on your system

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd encode-rs
```

2. Create a PostgreSQL database:
```bash
createdb encode
```

3. Configure the application by editing `config.toml`:
```toml
[database]
url = "postgresql://localhost/encode"

[ffmpeg]
path = "ffmpeg"
default_codec = "libx264"
default_preset = "medium"
default_crf = 23

[worker]
poll_interval_seconds = 5
```

4. Build the project:
```bash
cargo build --release
```

## Usage

### Adding Jobs

Add a video encoding job to the queue:

```bash
# Basic usage with defaults
cargo run -- add input.mp4 output.mp4

# With custom codec
cargo run -- add input.mp4 output.mp4 --codec libx265

# With all options
cargo run -- add input.mp4 output.mp4 --codec libx264 --preset slow --crf 20
```

### Running Workers

Start a worker to process jobs:

```bash
# Continuous mode (default)
cargo run -- worker

# Burst mode (exits when no more jobs)
cargo run -- worker --burst
```

You can run multiple workers simultaneously for parallel processing.

## Architecture

### Database Schema

The system uses PostgreSQL with the following jobs table:

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Primary key, auto-generated |
| `status` | VARCHAR(20) | Job status: pending, processing, done, failed |
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
│   ├── cli.rs       # CLI argument parsing
│   ├── config.rs    # Configuration management
│   ├── db.rs        # Database operations
│   ├── ffmpeg.rs    # FFmpeg integration
│   └── worker.rs    # Worker logic
├── migrations/
│   └── 001_initial.sql  # Database migrations
├── config.toml      # Application configuration
└── Cargo.toml       # Rust dependencies
```

### Modules

#### `cli.rs`
Command-line interface using `clap`. Supports:
- `add` command with options for input/output paths, codec, preset, and CRF
- `worker` command with optional burst mode

#### `config.rs`
TOML-based configuration loading for database, FFmpeg, and worker settings.

#### `db.rs`
Database abstraction layer with PostgreSQL operations:
- Connection pooling with `sqlx`
- Automatic migrations
- Job CRUD operations
- Safe job claiming with row-level locking

#### `ffmpeg.rs`
FFmpeg integration for video transcoding:
- Validates input files
- Creates output directories
- Executes FFmpeg with configurable parameters
- Error handling and logging

#### `worker.rs`
Worker process logic:
- Polls database for pending jobs
- Processes jobs using FFmpeg
- Updates job status
- Supports continuous and burst modes

## Configuration

### Database

```toml
[database]
url = "postgresql://username:password@host/database"
```

Supports standard PostgreSQL connection strings.

### FFmpeg

```toml
[ffmpeg]
path = "ffmpeg"           # Path to FFmpeg binary
default_codec = "libx264" # Default video codec
default_preset = "medium" # Default encoding preset
default_crf = 23         # Default quality (0-51)
```

Common codecs:
- `libx264` - H.264/AVC (best compatibility)
- `libx265` - H.265/HEVC (better compression)
- `libsvtav1` - AV1 (modern, best compression)
- `libvpx-vp9` - VP9 (good for web)

Common presets (slow to ultrafast):
- `veryslow` - Best quality, slowest
- `slow` - Good quality
- `medium` - Balanced (default)
- `fast` - Faster encoding

### Worker

```toml
[worker]
poll_interval_seconds = 5  # Seconds between job polls
```

## Development

### Build

```bash
cargo build
cargo build --release
```

### Testing

```bash
cargo test
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run Clippy lints
cargo clippy --all-targets --all-features

# Check formatting
cargo fmt -- --check
```

### Environment Variables

- `RUST_LOG` - Logging level (e.g., `info`, `debug`, `trace`)

Example:
```bash
RUST_LOG=debug cargo run -- worker
```

## Deployment

### Docker

Example Dockerfile:

```dockerfile
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ffmpeg postgresql-client
COPY --from=builder /app/target/release/encode /usr/local/bin/
COPY config.toml .
CMD ["encode", "worker"]
```

### Systemd Service

Example service file:

```ini
[Unit]
Description=Video Encoding Worker
After=network.target postgresql.service

[Service]
Type=simple
User=encode
WorkingDirectory=/opt/encode
ExecStart=/opt/encode/encode worker
Restart=always
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

## License

MIT License - See LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Troubleshooting

### FFmpeg not found
Ensure FFmpeg is installed and in your PATH, or specify the full path in `config.toml`.

### Database connection errors
- Verify PostgreSQL is running
- Check the connection URL in `config.toml`
- Ensure the database exists
- Verify user permissions

### Jobs stuck in "processing"
If a worker crashes, jobs may be stuck. Manually reset them:
```sql
UPDATE jobs SET status = 'pending' WHERE status = 'processing';
```

## Future Enhancements

- [ ] Job progress tracking
- [ ] Web dashboard for monitoring
- [ ] Retry logic with exponential backoff
- [ ] Priority queues
- [ ] Multiple output formats per job
- [ ] Hardware acceleration support (NVENC, QSV, VAAPI)
- [ ] REST API for job management
