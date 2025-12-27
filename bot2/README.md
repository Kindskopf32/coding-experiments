# Bot2 - Go Version

This is a Go port of the Python code review bot. It fetches pull request diffs from Gitea, sends them to OpenRouter for AI code review, and posts the review as a comment to an issue.

## Features

- Fetch pull request diffs from Gitea API
- Send diffs to OpenRouter API for AI-powered code review
- Post review comments to Gitea issues
- Configurable model selection
- Verbose mode for debugging

## Prerequisites

- Go 1.21 or later
- Gitea API token (set as `GITEA_TOKEN` environment variable)
- OpenRouter API token (set as `OPENROUTER_TOKEN` environment variable)

## Installation

```bash
cd bot2
go build -o bot2
```

## Usage

```bash
./bot2 -p <pr-number> -i <issue-number> [-m <model>] [-v]
```

### Arguments

- `-p, --pr-number`: Pull request number to review (required)
- `-i, --issue-number`: Issue number to comment on (required)
- `-m, --model`: OpenRouter model to use (default: `z-ai/glm-4.7`)
- `-v, --verbose`: Print detailed response information

### Example

```bash
export GITEA_TOKEN="your-gitea-token"
export OPENROUTER_TOKEN="your-openrouter-token"

./bot2 -p 123 -i 456 -m z-ai/glm-4.7 -v
```

## Development

### Running directly with `go run`

```bash
go run main.go -p 123 -i 456
```

### Project Structure

```
bot2/
├── go.mod      # Go module definition
├── main.go     # Main application code
└── README.md   # This file
```

## Differences from Python Version

- Uses Go's standard library (`net/http`, `encoding/json`, `flag`, `os`)
- Single-file implementation for simplicity
- Structured types for API requests/responses
- Custom error type with wrapping support
