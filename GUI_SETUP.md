# Cara Menggunakan GUI Version

## Setup

1. **Uncomment dependencies di Cargo.toml:**
   Edit `Cargo.toml` dan uncomment bagian GUI:
   ```toml
   eframe = "0.27"
   egui = "0.27"
   ```

2. **Build dan run:**
   ```bash
   cargo run --bin rust_app_gui
   ```

## Perbandingan CLI vs GUI

| Feature | CLI Version | GUI Version |
|---------|-------------|-------------|
| **Command** | `cargo run -- add "note"` | `cargo run --bin rust_app_gui` |
| **Interface** | Terminal/Command line | Window dengan buttons |
| **Dependencies** | Minimal | Lebih banyak (egui) |
| **Bundle Size** | ~1-2 MB | ~5-10 MB |

## Framework Lainnya

Lihat `RUST_UI_FRAMEWORKS.md` untuk opsi framework lainnya seperti:
- **Iced** - Modern desktop apps
- **Tauri** - Web-based (seperti Electron)
- **Dioxus** - React-like

