// Web API Server untuk Note Manager
// Untuk menjalankan: cargo run --bin rust_app_web
// Buka browser ke: http://localhost:3000

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

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

#[derive(Debug, Deserialize)]
struct CreateNoteRequest {
    content: String,
}

impl Notes {
    fn new() -> Self {
        Notes {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    #[allow(dead_code)]
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

// API Handlers
async fn get_notes(state: axum::extract::State<Arc<Mutex<Notes>>>) -> Json<Vec<Note>> {
    let notes = state.lock().unwrap();
    Json(notes.notes.clone())
}

async fn create_note(
    state: axum::extract::State<Arc<Mutex<Notes>>>,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<Json<Note>, StatusCode> {
    let mut notes = state.lock().unwrap();
    let note = Note {
        id: notes.next_id,
        content: payload.content,
        created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    notes.notes.push(note.clone());
    notes.next_id += 1;
    
    if save_notes(&notes).is_ok() {
        Ok(Json(note))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn delete_note(
    state: axum::extract::State<Arc<Mutex<Notes>>>,
    Path(id): Path<usize>,
) -> Result<StatusCode, StatusCode> {
    let mut notes = state.lock().unwrap();
    if notes.delete(id) {
        if save_notes(&notes).is_ok() {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() {
    let notes = Arc::new(Mutex::new(load_notes()));

    // Build router
    let app = Router::new()
        .route("/", get(index))
        .route("/api/notes", get(get_notes))
        .route("/api/notes", post(create_note))
        .route("/api/notes/:id", delete(delete_note))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(notes);

    println!("🚀 Server running on http://localhost:3000");
    println!("📝 Open http://localhost:3000 in your browser");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

