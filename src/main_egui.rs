// Contoh Note Manager dengan GUI menggunakan egui
// Untuk menjalankan: cargo run --bin rust_app_egui

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

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

struct NoteApp {
    notes: Notes,
    new_note_text: String,
    selected_id: Option<usize>,
}

impl NoteApp {
    fn new() -> Self {
        Self {
            notes: load_notes(),
            new_note_text: String::new(),
            selected_id: None,
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel dengan title
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("📝 Note Manager");
            ui.separator();
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);

                // Input area untuk note baru
                ui.horizontal(|ui| {
                    ui.label("New Note:");
                    ui.text_edit_singleline(&mut self.new_note_text);
                    if ui.button("➕ Add").clicked() {
                        if !self.new_note_text.trim().is_empty() {
                            self.notes.add(self.new_note_text.clone());
                            self.new_note_text.clear();
                            let _ = save_notes(&self.notes);
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // List notes
                if self.notes.notes.is_empty() {
                    ui.label("No notes yet. Add one above! 👆");
                } else {
                    ui.heading("Your Notes:");
                    ui.add_space(5.0);

                    egui::ScrollArea::vertical()
                        .max_height(400.0)
                        .show(ui, |ui| {
                            for note in &self.notes.notes.clone() {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(
                                                egui::RichText::new(&note.content)
                                                    .size(14.0)
                                                    .strong(),
                                            );
                                            ui.label(
                                                egui::RichText::new(&note.created_at)
                                                    .size(10.0)
                                                    .weak(),
                                            );
                                        });

                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button("🗑️ Delete").clicked() {
                                                self.notes.delete(note.id);
                                                let _ = save_notes(&self.notes);
                                            }
                                        });
                                    });
                                });
                                ui.add_space(5.0);
                            }
                        });
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_title("Note Manager - Rust GUI"),
        ..Default::default()
    };

    eframe::run_native(
        "Note Manager",
        options,
        Box::new(|_cc| Box::new(NoteApp::new())),
    )
}

