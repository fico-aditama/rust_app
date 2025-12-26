// Versi Terminal UI (TUI) - Lebih ringan dan compatible dengan Rust lama
// Untuk menjalankan: cargo run --bin rust_app_tui
// 
// Note: Versi ini menggunakan terminal interface, bukan GUI window
// Cocok untuk Rust 1.75+ tanpa perlu dependency berat

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

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

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_header() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║          📝 Note Manager - Terminal UI                ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
}

fn print_menu() {
    println!("Commands:");
    println!("  1. Add note");
    println!("  2. List notes");
    println!("  3. Delete note");
    println!("  4. Exit");
    print!("\nChoose option (1-4): ");
    io::stdout().flush().unwrap();
}

fn add_note_interactive(notes: &mut Notes) {
    print!("Enter note content: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let content = input.trim().to_string();
    
    if !content.is_empty() {
        notes.add(content);
        if save_notes(notes).is_ok() {
            println!("✅ Note added successfully!");
        } else {
            println!("❌ Error saving note");
        }
    } else {
        println!("❌ Note cannot be empty");
    }
}

fn list_notes(notes: &Notes) {
    if notes.notes.is_empty() {
        println!("No notes found.");
        return;
    }
    
    println!("\n📝 Your Notes:");
    println!("{}", "═".repeat(60));
    for note in &notes.notes {
        println!("[{}] {}", note.id, note.content);
        println!("    Created: {}", note.created_at);
        println!();
    }
}

fn delete_note_interactive(notes: &mut Notes) {
    list_notes(notes);
    if notes.notes.is_empty() {
        return;
    }
    
    print!("Enter note ID to delete: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim().parse::<usize>() {
        Ok(id) => {
            if notes.delete(id) {
                if save_notes(notes).is_ok() {
                    println!("✅ Note {} deleted successfully!", id);
                } else {
                    println!("❌ Error saving changes");
                }
            } else {
                println!("❌ Note with ID {} not found", id);
            }
        }
        Err(_) => {
            println!("Error: Invalid ID. Please provide a number");
        }
    }
}

fn main() {
    let mut notes = load_notes();
    
    loop {
        clear_screen();
        print_header();
        list_notes(&notes);
        println!();
        print_menu();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();
        
        match choice {
            "1" => {
                clear_screen();
                print_header();
                add_note_interactive(&mut notes);
                print!("\nPress Enter to continue...");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut String::new()).unwrap();
            }
            "2" => {
                clear_screen();
                print_header();
                list_notes(&notes);
                print!("\nPress Enter to continue...");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut String::new()).unwrap();
            }
            "3" => {
                clear_screen();
                print_header();
                delete_note_interactive(&mut notes);
                print!("\nPress Enter to continue...");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut String::new()).unwrap();
            }
            "4" => {
                println!("Goodbye! 👋");
                break;
            }
            _ => {
                println!("Invalid option. Please choose 1-4.");
                print!("\nPress Enter to continue...");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }
}

