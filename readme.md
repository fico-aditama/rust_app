# Rust Note Manager

A simple CLI note-taking application built with Rust. This project demonstrates:
- Structs and enums
- File I/O operations
- JSON serialization/deserialization
- Error handling
- Command-line argument parsing
- Ownership and borrowing concepts

## Features

- ✅ Add notes with automatic timestamps
- 📋 List all your notes
- 🗑️ Delete notes by ID
- 💾 Persistent storage (saves to `notes.json`)

## Building

```bash
cargo build --release
```

## Running

### CLI Version (Command Line)
```bash
# Add a note
cargo run -- add "Remember to buy groceries"

# List all notes
cargo run -- list

# Delete a note by ID
cargo run -- delete 1

# Show help
cargo run -- help
```

### TUI Version (Terminal UI - Interactive)
```bash
# Run interactive terminal UI
cargo run --bin rust_app_tui
```

### Web Version (Browser-based - Recommended! 🌐)
```bash
# Run web server
cargo run --bin rust_app_web

# Buka browser ke: http://localhost:3000
```

**Features:**
- ✅ Modern web UI dengan HTML/CSS/JavaScript
- ✅ REST API untuk CRUD operations
- ✅ Real-time updates
- ✅ Beautiful gradient design

### GUI Version (Window-based - Requires Rust 1.76+)
```bash
# Uncomment egui dependencies in Cargo.toml first
# Then run:
cargo run --bin rust_app_gui
```

**Note:** GUI version memerlukan Rust yang lebih baru. Jika Rust 1.75, gunakan TUI atau Web version yang sudah compatible!

## Project Structure

```
rust_app/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs        # Main application code
└── notes.json         # Auto-generated file for storing notes
```

## Learning Points

This project covers several important Rust concepts:
- **Structs**: `Note` and `Notes` structs to organize data
- **Traits**: Using `Serialize` and `Deserialize` from serde
- **Error Handling**: Using `Result` and `Option` types
- **Ownership**: Understanding how Rust manages memory
- **File I/O**: Reading and writing JSON files
- **Command-line parsing**: Using `std::env::args()`

