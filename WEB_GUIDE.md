# 🌐 Web Version Guide

Aplikasi Note Manager versi web menggunakan Rust backend dengan Axum framework.

## 🚀 Quick Start

```bash
# Build dan run
cargo run --bin rust_app_web

# Buka browser
# http://localhost:3000
```

## 📡 API Endpoints

### GET `/api/notes`
Mendapatkan semua notes
```bash
curl http://localhost:3000/api/notes
```

### POST `/api/notes`
Membuat note baru
```bash
curl -X POST http://localhost:3000/api/notes \
  -H "Content-Type: application/json" \
  -d '{"content": "My new note"}'
```

### DELETE `/api/notes/:id`
Menghapus note berdasarkan ID
```bash
curl -X DELETE http://localhost:3000/api/notes/1
```

## 🏗️ Architecture

```
┌─────────────┐
│   Browser   │
│  (Frontend) │
└──────┬──────┘
       │ HTTP
       ▼
┌─────────────┐
│   Axum      │
│  (Backend)  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ notes.json  │
│  (Storage)  │
└─────────────┘
```

## 📁 File Structure

```
rust_app/
├── src/
│   └── main_web.rs      # Web server dengan Axum
├── static/
│   └── index.html       # Frontend HTML/CSS/JS
└── notes.json           # Data storage (shared dengan CLI/TUI)
```

## 🛠️ Tech Stack

- **Backend**: Axum (async web framework)
- **Frontend**: Vanilla HTML/CSS/JavaScript
- **Storage**: JSON file (notes.json)
- **Async Runtime**: Tokio

## 🔧 Development

### Menambah Fitur Baru

1. **Backend (Rust)**: Edit `src/main_web.rs`
2. **Frontend (HTML/JS)**: Edit `static/index.html`

### Menambah Route Baru

```rust
let app = Router::new()
    .route("/api/new-endpoint", get(handler_function))
    .with_state(shared_state);
```

## 🌍 Deployment

### Build untuk Production

```bash
cargo build --release --bin rust_app_web
./target/release/rust_app_web
```

### Docker (Optional)

Bisa dibuat Dockerfile untuk deploy ke production server.

## 🎨 Customization

### Mengubah Port

Edit `src/main_web.rs`:
```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
```

### Mengubah Styling

Edit CSS di `static/index.html` bagian `<style>`.

## 📝 Notes

- Web version share file `notes.json` yang sama dengan CLI/TUI version
- Server menggunakan CORS permissive untuk development
- Untuk production, configure CORS dengan lebih strict

