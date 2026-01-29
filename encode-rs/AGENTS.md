# Agent Guidelines for encode-rs

This is a Rust-based video encoding job queue system using PostgreSQL and FFmpeg.

## Project Overview

- **Language**: Rust (Edition 2024)
- **Binary name**: `encode`
- **Architecture**: Async CLI tool with worker process

## Build Commands

```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the binary
cargo run -- <args>
# Example: cargo run -- add input.mp4 output.mp4 --codec libx264
```

## Test Commands

```bash
# Run all tests
cargo test

# Run a specific test by name
cargo test <test_name>

# Run tests in a specific module
cargo test <module_name>::

# Run with output visible
cargo test -- --nocapture

# Run only tests matching a pattern
cargo test <pattern>
```

## Lint and Format Commands

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

## Code Style Guidelines

### Imports
- Group imports in order:
  1. `std` library imports
  2. External crate imports
  3. Internal `crate::` imports (one blank line after external)
- Example:
```rust
use std::path::Path;

use anyhow::Result;
use tokio::process::Command;
use tracing::{debug, error, info};

use crate::db::Job;
```

### Error Handling
- Use `anyhow::Result<T>` for function return types
- Use `anyhow::Context` for adding context to errors
- Prefer `?` operator over explicit match on Results
- Create errors with `anyhow::anyhow!("message")`

### Naming Conventions
- **Functions/Variables**: `snake_case`
- **Types/Structs/Enums**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
- **Macro-generated**: Follow Rust conventions

### Async Patterns
- Use `tokio` for async runtime
- Mark async functions with `async fn`
- Use `.await` for async calls
- Main function: `#[tokio::main] async fn main() -> Result<()>`

### Logging
- Use `tracing` crate for logging
- Log levels: `trace!`, `debug!`, `info!`, `warn!`, `error!`
- Include contextual information in log messages
- Use structured logging when appropriate

### Types and Structs
- Derive common traits: `#[derive(Debug)]` minimum
- Use `Deserialize` from serde for config types
- Use `thiserror` if creating custom error types (project currently uses anyhow)

### Documentation
- Add doc comments (`///`) for public APIs
- Use `//!` for module-level documentation
- Document complex business logic with inline comments

### Database (sqlx)
- Use raw SQL with `sqlx::query` for simple queries
- Use transactions (`pool.begin()`) for multi-step operations
- Use `FOR UPDATE SKIP LOCKED` for job claiming
- Always use parameterized queries (`.bind()`)

### General Principles
- Prefer immutability (`let` over `let mut`)
- Use `&str` for string parameters when possible
- Use `Path` and `PathBuf` for file paths
- Keep functions focused and under 50 lines when possible
- Handle all Result types (use `?` or explicit match)
