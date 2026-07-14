# Oxide Flix 🦀

A lightweight local video streaming player built with Rust + Axum. It automatically scans a specified directory for video files and provides a Web interface for browsing, playing, and managing them.

## Preview

```
┌─────────────────────────────────────────────┐
│  🦀 Oxide Flix                                │
├─────────────────────────────────────────────┤
│  📁 Root / Movies / Sci-Fi                   │
├─────────────────────────────────────────────┤
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐      │
│  │ 📁   │ │ 📁   │ │ 🎬  │ │ 🎬  │      │
│  │Movies│ │  TV  │ │alien│ │ dune│      │
│  │ Folder│ │Folder│ │1.2GB│ │ 2.1GB│      │
│  └──────┘ └──────┘ └──────┘ └──────┘      │
├─────────────────────────────────────────────┤
│  ┌─────────────────────────────────────┐    │
│  │           ▶ Video Player             │    │
│  └─────────────────────────────────────┘    │
│  ┌─────────────────────────────────────┐    │
│  │ 🎬 alien.mp4                        │    │
│  │ Name: alien.mp4     Size: 1.2 GB   │    │
│  │ Type: video/mp4     Mode: html5    │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

## Features

- 📂 **File Browser** — Grid-based display of video files and subdirectories with breadcrumb navigation
- ▶️ **Video Playback** — HTML5 `<video>` streaming with seek support (HTTP Range requests)
- 📋 **Metadata Display** — Shows filename, size, MIME type and other basic info
- 🗑️ **Delete Management** — Delete button appears on hover; supports removing files or entire folders
- 🔒 **Path Security** — All file operations go through path sanitization to prevent directory traversal attacks
- 🌐 **Responsive Layout** — Adapts to different screen sizes

## Supported Video Formats

| Format | Extension |
|--------|-----------|
| MP4  | `.mp4` |
| MKV  | `.mkv` |
| WebM | `.webm` |
| AVI  | `.avi` |
| MOV  | `.mov` |
| WMV  | `.wmv` |
| FLV  | `.flv` |
| M4V  | `.m4v` |
| TS   | `.ts`  |
| M3U8 | `.m3u8` |

> Actual playback capability depends on the browser's codec support. Chrome / Edge are recommended for best compatibility.

## Quick Start

### Prerequisites

- Rust toolchain (edition 2024)

### Build & Run

```bash
# Build
cargo build --release

# Run (using current directory as media root)
./target/release/oxide-flix-rs

# Specify a media directory
./target/release/oxide-flix-rs --data-dir /path/to/your/videos

# Specify a port
./target/release/oxide-flix-rs --port 8080
```

Open your browser and navigate to `http://127.0.0.1:1000`.

### Configuration

| Argument | Environment Variable | Default | Description |
|----------|--------------------|---------|-------------|
| `--ipv4` | `APP_IPV4` | `0.0.0.0` | Listening address |
| `--port` | `APP_PORT` | `1000` | Listening port |
| `--data-dir` | `APP_DATA_DIR` | Current directory | Media file root directory |

## API Reference

All API endpoints return JSON and are protected by path sanitization.

### `GET /` — Home page

Returns the Web interface.

### `GET /api/video/list` — List root directory

Lists files and folders under `APP_DATA_DIR`.

### `GET /api/video/list/{*path}` — List subdirectory

Lists files and folders under a specified subdirectory.

**Example response:**
```json
{
  "entries": [
    { "name": "Movies", "is_dir": true, "size_bytes": 0 },
    { "name": "video.mp4", "is_dir": false, "size_bytes": 123456789 }
  ],
  "current_path": "Movies"
}
```

### `GET /api/video/info/{*path}` — Video metadata

Returns basic information about a video file.

### `GET /api/video/stream/{*path}` — Video stream

Streams a video file with HTTP Range header support for seeking.

### `DELETE /api/video/delete/{*path}` — Delete

Deletes a file or folder (folders are removed recursively).

## Project Structure

```
oxide-flix-rs/
├── Cargo.toml          # Dependencies and project config
├── src/
│   └── main.rs         # Server entry: routes, handlers, path sanitization
├── templates/
│   └── index.html      # Frontend: file browser + video player
└── README.md
```

## Tech Stack

- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Web framework) + [Tokio](https://tokio.rs) (async runtime)
- **Frontend**: Vanilla HTML + CSS + JavaScript (zero framework dependencies)
- **Templating**: [Askama](https://github.com/rust-askama/askama) (server-side template engine)
- **CLI**: [Clap](https://github.com/clap-rs/clap) (argument parsing)
