# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Wyrmspan Points Tracker is a web application for tracking scores in the board game _Wyrmspan_. It's built with Rust using the Axum web framework and serves a single-page application with a simple score submission and leaderboard system.

**Key characteristics:**
- Minimal Rust web server with ~95 lines of code in `src/main.rs`
- Stateless backend - all data persists to `scores.txt` in configurable directory (defaults to `/data`)
- Data directory configurable via `DATA_DIR` environment variable
- Static frontend (HTML/CSS/JS) served from `/static`
- Designed for containerized deployment (Docker/Podman) with mountable data volume
- Single-page app accessed via web browser (typically on phones/tablets during gameplay)

## Architecture

### Backend (Rust/Axum)
- **Entry point:** `src/main.rs` - contains entire backend logic
- **Routes:**
  - `GET /` - Reads top 5 scores from `${DATA_DIR}/scores.txt`, injects into HTML template, returns rendered page
  - `POST /submit` - Accepts JSON array of `{name, total}` objects, appends to scores file
  - `/static/*` - Serves static assets via tower-http
- **Data storage:** Flat file at `${DATA_DIR}/scores.txt` with format `PlayerName: score` (one per line)
- **Configuration:** Data directory set via `DATA_DIR` environment variable (defaults to `/data`)
- **Score processing:** In-memory sorting on read, keeps top 5 only for display

### Frontend (Static HTML/CSS/JS)
- **`static/scores.html`** - Main UI template with score entry form and leaderboard
- **`static/styles.css`** - Styling
- **`static/ai-generated-background.png`** - Background image

### Data Flow
1. User requests `/` → Backend reads `${DATA_DIR}/scores.txt` → Sorts and gets top 5 → Injects into HTML template → Returns page
2. User submits scores → Frontend POSTs JSON to `/submit` → Backend appends to `${DATA_DIR}/scores.txt`
3. No database, no sessions - purely file-based persistence
4. Data directory location is read from `DATA_DIR` environment variable (defaults to `/data`)

## Common Commands

### Development
```bash
# Build the project
cargo build

# Run locally (listens on 0.0.0.0:3000)
cargo run

# Access app during development
# http://127.0.0.1:3000

# Build optimized release binary
cargo build --release
```

### Testing
```bash
# Run tests (if any are added)
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Container Development
```bash
# Build Docker image (requires release binary first)
cargo build --release
docker build -t wyrmspan-points-tracker:local .

# Run container locally (with ephemeral data)
docker run -p 3000:3000 wyrmspan-points-tracker:local

# Run container with persistent storage (recommended)
docker run -p 3000:3000 -v /path/to/host/data:/data wyrmspan-points-tracker:local

# Run container with custom data directory
docker run -p 3000:3000 -e DATA_DIR=/custom/path -v /path/to/host/data:/custom/path wyrmspan-points-tracker:local

# Or with Podman
podman build -t wyrmspan-points-tracker:local .
podman run -p 3000:3000 -v /path/to/host/data:/data wyrmspan-points-tracker:local

# Test published container
docker run -p 3000:3000 -v /path/to/host/data:/data ghcr.io/grimvoodoo/wyrmspan-points-tracker:latest
```

## Development Workflow

### Making Changes

1. **Backend changes:** Edit `src/main.rs` directly - all server logic is in this single file
2. **Frontend changes:** Edit files in `static/` directory
3. **Data format changes:** Modify `scores.txt` parsing/writing logic in `get_top_scores()` and `submit_scores()` functions
4. **Configuration changes:** Data directory is controlled by `get_scores_file_path()` function which reads `DATA_DIR` env var

### Release Process

**Automatic:** Pushing to `main` branch triggers `.github/workflows/release.yaml` which:
1. Builds release binary
2. Extracts version from `Cargo.toml`
3. Bumps patch version using `cargo-bump`
4. Creates git tag with version
5. Builds and pushes container images to `ghcr.io/grimvoodoo/wyrmspan-points-tracker`

**Manual version bumps:** Edit version in `Cargo.toml` before pushing to main

### File Structure Notes

- Data directory defaults to `/data` but can be overridden with `DATA_DIR` environment variable
- `scores-mine.txt` is gitignored (for local testing without affecting production scores)
- Container creates `/data/scores.txt` with empty initial state
- Container runs as non-root user `appuser` for security
- Application listens on `0.0.0.0:3000` (container exposes port 3000)

### Environment Variables

- **`DATA_DIR`** - Directory where `scores.txt` will be stored (default: `/data`)
  - Example: `DATA_DIR=/custom/path` will store scores at `/custom/path/scores.txt`
  - Useful for development or custom deployment scenarios

## Key Implementation Details

### Score Entry Data Structure
```rust
struct ScoreEntry {
    username: String,
    total: u32,
}
```

### Score File Format
```
PlayerName: 42
AnotherPlayer: 38
```

### Frontend Submission Format (JSON)
```json
[
  {"name": "Player1", "total": 42},
  {"name": "Player2", "total": 38}
]
```

### Important Behaviors
- Empty player names are filtered out on submission
- Scores are sorted descending (highest first)
- Only top 5 scores displayed
- File I/O errors print to stderr but don't crash the server
- Score file grows indefinitely (no cleanup of old scores)
