# Rust Note Manager

A simple note-taking application built with Rust, demonstrating various Rust concepts and multiple UI implementations.

## Features

- ✅ Add notes with automatic timestamps
- 📋 List all your notes
- 🗑️ Delete notes by ID
- 💾 Persistent storage (saves to `notes.json`)
- 🌐 Web interface
- 🖥️ Terminal UI
- 📝 Command-line interface
- 🤖 Object Detection (Python & Rust examples)

## Quick Start

### Web Version (Recommended)
```bash
cargo run --bin rust_app_web
# Open http://localhost:3000
```

### Terminal UI
```bash
cargo run --bin rust_app_tui
```

### CLI Version
```bash
cargo run -- add "My note"
cargo run -- list
cargo run -- delete 1
```

## Object Detection

### Python Version
```bash
# Install dependencies
pip install ultralytics opencv-python pillow

# Detect objects in image
python object_detection.py image photo.jpg output.jpg

# Detect objects in video
python object_detection.py video video.mp4 output.mp4

# Real-time webcam detection
python object_detection.py webcam
```

### Rust Version
```bash
# Uncomment dependencies in Cargo.toml:
# ort = "2.0"
# image = "0.24"

# Run detection
cargo run --bin rust_object_detection -- image photo.jpg results.json
```

## Project Structure

```
rust_app/
├── src/
│   ├── main.rs                    # CLI version
│   ├── main_tui.rs                # Terminal UI
│   ├── main_web.rs                # Web server
│   ├── main_object_detection.rs   # Object detection CLI
│   └── object_detection.rs        # Object detection module
├── static/
│   └── index.html                 # Web frontend
├── object_detection.py            # Python object detection
├── Cargo.toml                     # Rust dependencies
└── notes.json                     # Data storage
```

## Learning Points

This project demonstrates:
- **Structs & Enums**: Data organization
- **Error Handling**: Result and Option types
- **Ownership**: Memory management
- **File I/O**: JSON serialization
- **Async Programming**: Tokio and Axum
- **Web Development**: REST API with Rust
- **Machine Learning**: Object detection examples

## Requirements

- Rust 1.75+ (for CLI, TUI, Web versions)
- Rust 1.76+ (for GUI/Linux desktop versions)
- Python 3.8+ (for Python object detection)

## License

MIT

