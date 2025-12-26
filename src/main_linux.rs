// Linux Desktop App dengan GTK-rs
// Untuk menjalankan: cargo run --bin rust_app_linux
//
// Prerequisites:
// sudo apt-get install libgtk-4-dev libadwaita-1-dev

use gtk::prelude::*;
use adw::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Note {
    id: usize,
    content: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Notes {
    notes: Vec<Note>,
    next_id: usize,
}

impl Notes {
    fn new() -> Self {
        Notes {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, content: String) {
        let note = Note {
            id: self.next_id,
            content,
            created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        self.notes.push(note);
        self.next_id += 1;
    }

    fn delete(&mut self, id: usize) -> bool {
        let initial_len = self.notes.len();
        self.notes.retain(|note| note.id != id);
        self.notes.len() < initial_len
    }
}

fn load_notes() -> Notes {
    match fs::read_to_string("notes.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Notes::new()),
        Err(_) => Notes::new(),
    }
}

fn save_notes(notes: &Notes) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(notes)?;
    fs::write("notes.json", json)?;
    Ok(())
}

fn build_ui(app: &adw::Application) {
    let notes = Arc::new(Mutex::new(load_notes()));

    // Create main window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("📝 Note Manager")
        .default_width(600)
        .default_height(700)
        .build();

    // Create header bar
    let header = adw::HeaderBar::new();
    window.set_titlebar(Some(&header));

    // Create main box
    let main_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .margin_start(20)
        .margin_end(20)
        .margin_top(20)
        .margin_bottom(20)
        .build();

    // Input section
    let input_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .build();

    let entry = gtk::Entry::builder()
        .placeholder_text("Tulis note baru di sini...")
        .hexpand(true)
        .build();

    let add_button = gtk::Button::builder()
        .label("➕ Add")
        .css_classes(vec!["suggested-action"])
        .build();

    input_box.append(&entry);
    input_box.append(&add_button);

    // Scrolled window for notes list
    let scrolled = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let notes_list = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .build();

    scrolled.set_child(Some(&notes_list));

    // Function to refresh notes list
    let refresh_notes = {
        let notes = Arc::clone(&notes);
        let notes_list = notes_list.clone();
        Rc::new(move || {
            // Clear existing children
            while let Some(child) = notes_list.first_child() {
                notes_list.remove(&child);
            }

            let notes_guard = notes.lock().unwrap();
            if notes_guard.notes.is_empty() {
                let empty_label = gtk::Label::builder()
                    .text("No notes yet. Add one above! 👆")
                    .css_classes(vec!["title-3"])
                    .margin_top(50)
                    .build();
                notes_list.append(&empty_label);
            } else {
                for note in &notes_guard.notes {
                    let note_frame = gtk::Frame::builder()
                        .css_classes(vec!["card"])
                        .margin_start(10)
                        .margin_end(10)
                        .build();

                    let note_box = gtk::Box::builder()
                        .orientation(gtk::Orientation::Vertical)
                        .spacing(5)
                        .margin_start(15)
                        .margin_end(15)
                        .margin_top(15)
                        .margin_bottom(15)
                        .build();

                    let content_label = gtk::Label::builder()
                        .text(&note.content)
                        .css_classes(vec!["body"])
                        .halign(gtk::Align::Start)
                        .wrap(true)
                        .build();

                    let date_label = gtk::Label::builder()
                        .text(&format!("Created: {}", note.created_at))
                        .css_classes(vec!["caption"])
                        .halign(gtk::Align::Start)
                        .build();

                    let button_box = gtk::Box::builder()
                        .orientation(gtk::Orientation::Horizontal)
                        .halign(gtk::Align::End)
                        .build();

                    let delete_button = gtk::Button::builder()
                        .label("🗑️ Delete")
                        .css_classes(vec!["destructive-action"])
                        .build();

                    let note_id = note.id;
                    let notes_clone = Arc::clone(&notes);
                    let refresh_clone = refresh_notes.clone();
                    delete_button.connect_clicked(move |_| {
                        let mut notes_guard = notes_clone.lock().unwrap();
                        if notes_guard.delete(note_id) {
                            let _ = save_notes(&notes_guard);
                            refresh_clone();
                        }
                    });

                    button_box.append(&delete_button);
                    note_box.append(&content_label);
                    note_box.append(&date_label);
                    note_box.append(&button_box);
                    note_frame.set_child(Some(&note_box));
                    notes_list.append(&note_frame);
                }
            }
        })
    };

    // Add button click handler
    {
        let notes = Arc::clone(&notes);
        let entry_clone = entry.clone();
        let refresh_clone = refresh_notes.clone();
        add_button.connect_clicked(move |_| {
            let content = entry_clone.text().to_string();
            if !content.trim().is_empty() {
                let mut notes_guard = notes.lock().unwrap();
                notes_guard.add(content);
                let _ = save_notes(&notes_guard);
                entry_clone.set_text("");
                refresh_clone();
            }
        });
    }

    // Enter key handler
    {
        let notes = Arc::clone(&notes);
        let entry_clone = entry.clone();
        let refresh_clone = refresh_notes.clone();
        entry.connect_activate(move |_| {
            let content = entry_clone.text().to_string();
            if !content.trim().is_empty() {
                let mut notes_guard = notes.lock().unwrap();
                notes_guard.add(content);
                let _ = save_notes(&notes_guard);
                entry_clone.set_text("");
                refresh_clone();
            }
        });
    }

    // Initial load
    refresh_notes();

    main_box.append(&input_box);
    main_box.append(&scrolled);

    window.set_content(Some(&main_box));
    window.present();
}

fn main() {
    let app = adw::Application::builder()
        .application_id("com.rust_app.note_manager")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

