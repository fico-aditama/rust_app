# Cara Update Rust untuk GUI Support

Masalah yang terjadi: Rust 1.75.0 terlalu lama dan beberapa dependency baru require Rust yang lebih baru.

## Solusi 1: Install Rustup (Recommended)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source ~/.cargo/env

# Update ke stable terbaru
rustup update stable

# Set default
rustup default stable
```

Setelah itu, coba lagi:
```bash
cargo run --bin rust_app_gui
```

## Solusi 2: Clear Cargo Registry Cache

Jika tidak bisa update Rust, coba clear cache yang corrupt:

```bash
# Hapus registry cache yang corrupt
rm -rf ~/.cargo/registry/src/index.crates.io-*

# Atau hapus semua cache
rm -rf ~/.cargo/registry

# Lalu build lagi
cargo build --bin rust_app_gui
```

## Solusi 3: Gunakan TUI Version (Lebih Ringan)

Lihat `src/main_tui.rs` untuk versi Terminal UI yang lebih ringan dan compatible dengan Rust 1.75.

