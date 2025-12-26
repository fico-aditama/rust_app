# Rust UI Frameworks - Panduan Lengkap

Rust memiliki beberapa framework untuk membuat UI! Berikut opsi-opsinya:

## 🎨 Framework Populer

### 1. **egui** - Immediate Mode GUI (Paling Mudah untuk Pemula)
- ✅ **Sederhana** - Immediate mode, tidak perlu state management kompleks
- ✅ **Portable** - Bisa native dan web
- ✅ **Cocok untuk**: Tools, debugger, simple apps
- ❌ **Kurang cocok untuk**: Complex desktop apps

**Contoh penggunaan:**
```rust
egui::Window::new("My Window").show(ctx, |ui| {
    ui.label("Hello World!");
    if ui.button("Click me").clicked() {
        println!("Clicked!");
    }
});
```

---

### 2. **Iced** - Elm-inspired (Modern & Type-safe)
- ✅ **Modern** - Functional reactive programming
- ✅ **Type-safe** - Compile-time safety
- ✅ **Cross-platform** - Windows, macOS, Linux, Web
- ✅ **Cocok untuk**: Desktop apps modern

**Contoh penggunaan:**
```rust
#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

struct App {
    counter: i32,
}

impl Application for App {
    type Message = Message;
    
    fn view(&self) -> Element<Message> {
        button("Increment").on_press(Message::ButtonPressed).into()
    }
}
```

---

### 3. **Tauri** - Web Tech + Rust Backend (Seperti Electron)
- ✅ **Kecil** - Bundle size jauh lebih kecil dari Electron
- ✅ **Familiar** - Pakai HTML/CSS/JS untuk UI
- ✅ **Secure** - Rust backend yang aman
- ✅ **Cocok untuk**: Desktop apps dengan web UI

**Cara kerja:**
- Frontend: HTML/CSS/JavaScript (React, Vue, dll)
- Backend: Rust (untuk logic berat)
- Bundle: ~5-10MB (vs Electron ~100MB+)

---

### 4. **Dioxus** - React-like untuk Rust
- ✅ **Familiar** - Syntax mirip React
- ✅ **Cross-platform** - Desktop, Web, Mobile
- ✅ **Cocok untuk**: Developer yang sudah kenal React

**Contoh:**
```rust
fn App() -> Element {
    rsx! {
        div {
            h1 { "Hello Dioxus!" }
            button { onclick: move |_| println!("Clicked!"),
                "Click me"
            }
        }
    }
}
```

---

### 5. **GTK-rs** - Native Linux GUI
- ✅ **Native** - Look & feel native Linux
- ✅ **Mature** - Sudah lama ada
- ❌ **Linux-focused** - Kurang bagus untuk cross-platform

---

### 6. **Bevy** - Game Engine (Bisa untuk UI juga)
- ✅ **Powerful** - ECS (Entity Component System)
- ✅ **Cocok untuk**: Games, visualizations
- ❌ **Overkill** untuk simple UI apps

---

## 📊 Perbandingan Cepat

| Framework | Kesulitan | Use Case | Bundle Size |
|-----------|-----------|----------|-------------|
| **egui** | ⭐ Mudah | Tools, simple apps | Kecil |
| **Iced** | ⭐⭐ Medium | Modern desktop apps | Sedang |
| **Tauri** | ⭐⭐ Medium | Web-based desktop apps | Sangat kecil |
| **Dioxus** | ⭐⭐ Medium | React-like apps | Sedang |
| **GTK-rs** | ⭐⭐⭐ Sulit | Native Linux apps | Besar |
| **Bevy** | ⭐⭐⭐ Sulit | Games, visualizations | Besar |

---

## 🚀 Rekomendasi untuk Pemula

### Mulai dengan **egui** jika:
- Ingin cepat prototipe
- Tidak perlu UI yang sangat kompleks
- Ingin belajar konsep GUI Rust

### Pilih **Iced** jika:
- Ingin buat desktop app modern
- Butuh type safety yang kuat
- Suka functional programming style

### Pilih **Tauri** jika:
- Sudah familiar dengan web development
- Ingin pakai React/Vue untuk UI
- Butuh bundle size kecil

---

## 💡 Contoh: Note Manager dengan egui

Lihat file `src/main_egui.rs` untuk contoh implementasi Note Manager dengan GUI!

