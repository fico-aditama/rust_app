use std::fs;
use std::io;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    fn list(&self) {
        if self.notes.is_empty() {
            println!("No notes found.");
            return;
        }
        println!("\n📝 Your Notes:");
        println!("{}", "=".repeat(50));
        for note in &self.notes {
            println!("[{}] {}", note.id, note.content);
            println!("    Created: {}", note.created_at);
            println!();
        }
    }

    fn delete(&mut self, id: usize) -> bool {
        let initial_len = self.notes.len();
        self.notes.retain(|note| note.id != id);
        self.notes.len() < initial_len
    }
}

fn load_notes() -> Notes {
    match fs::read_to_string("notes.json") {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| Notes::new())
        }
        Err(_) => Notes::new(),
    }
}

fn save_notes(notes: &Notes) -> io::Result<()> {
    let json = serde_json::to_string_pretty(notes)?;
    fs::write("notes.json", json)?;
    Ok(())
}

fn main() {
    let mut notes = load_notes();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Error: Please provide note content");
                println!("Usage: {} add \"Your note here\"", args[0]);
                return;
            }
            let content = args[2..].join(" ");
            notes.add(content);
            if save_notes(&notes).is_ok() {
                println!("✅ Note added successfully!");
            } else {
                println!("❌ Error saving note");
            }
        }
        "list" => {
            notes.list();
        }
        "delete" => {
            if args.len() < 3 {
                println!("Error: Please provide note ID");
                println!("Usage: {} delete <id>", args[0]);
                return;
            }
            match args[2].parse::<usize>() {
                Ok(id) => {
                    if notes.delete(id) {
                        if save_notes(&notes).is_ok() {
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
        "help" => {
            print_usage();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("📝 Note Manager - A simple CLI note-taking app");
    println!("\nUsage:");
    println!("  {} add \"Your note\"     - Add a new note", std::env::args().next().unwrap());
    println!("  {} list                - List all notes", std::env::args().next().unwrap());
    println!("  {} delete <id>         - Delete a note by ID", std::env::args().next().unwrap());
    println!("  {} help                - Show this help message", std::env::args().next().unwrap());
}

