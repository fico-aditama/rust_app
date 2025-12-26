import json
import sys
from datetime import datetime
from typing import List, Optional

class Note:
    def __init__(self, id: int, content: str, created_at: str):
        self.id = id
        self.content = content
        self.created_at = created_at

    def to_dict(self):
        return {
            "id": self.id,
            "content": self.content,
            "created_at": self.created_at
        }

    @classmethod
    def from_dict(cls, data: dict):
        return cls(data["id"], data["content"], data["created_at"])

class Notes:
    def __init__(self):
        self.notes: List[Note] = []
        self.next_id = 1

    def add(self, content: str):
        note = Note(
            id=self.next_id,
            content=content,
            created_at=datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        )
        self.notes.append(note)
        self.next_id += 1

    def list(self):
        if not self.notes:
            print("No notes found.")
            return
        print("\n📝 Your Notes:")
        print("=" * 50)
        for note in self.notes:
            print(f"[{note.id}] {note.content}")
            print(f"    Created: {note.created_at}")
            print()

    def delete(self, id: int) -> bool:
        initial_len = len(self.notes)
        self.notes = [note for note in self.notes if note.id != id]
        return len(self.notes) < initial_len

    def to_dict(self):
        return {
            "notes": [note.to_dict() for note in self.notes],
            "next_id": self.next_id
        }

    @classmethod
    def from_dict(cls, data: dict):
        notes = cls()
        notes.notes = [Note.from_dict(note_data) for note_data in data.get("notes", [])]
        notes.next_id = data.get("next_id", 1)
        return notes

def load_notes() -> Notes:
    try:
        with open("notes.json", "r") as f:
            data = json.load(f)
            return Notes.from_dict(data)
    except (FileNotFoundError, json.JSONDecodeError):
        return Notes()

def save_notes(notes: Notes) -> bool:
    try:
        with open("notes.json", "w") as f:
            json.dump(notes.to_dict(), f, indent=2)
        return True
    except Exception:
        return False

def print_usage():
    print("📝 Note Manager - A simple CLI note-taking app")
    print("\nUsage:")
    print(f"  {sys.argv[0]} add \"Your note\"     - Add a new note")
    print(f"  {sys.argv[0]} list                - List all notes")
    print(f"  {sys.argv[0]} delete <id>         - Delete a note by ID")
    print(f"  {sys.argv[0]} help                - Show this help message")

def main():
    notes = load_notes()
    args = sys.argv

    if len(args) < 2:
        print_usage()
        return

    command = args[1]

    if command == "add":
        if len(args) < 3:
            print("Error: Please provide note content")
            print(f"Usage: {args[0]} add \"Your note here\"")
            return
        content = " ".join(args[2:])
        notes.add(content)
        if save_notes(notes):
            print("✅ Note added successfully!")
        else:
            print("❌ Error saving note")

    elif command == "list":
        notes.list()

    elif command == "delete":
        if len(args) < 3:
            print("Error: Please provide note ID")
            print(f"Usage: {args[0]} delete <id>")
            return
        try:
            id = int(args[2])
            if notes.delete(id):
                if save_notes(notes):
                    print(f"✅ Note {id} deleted successfully!")
                else:
                    print("❌ Error saving changes")
            else:
                print(f"❌ Note with ID {id} not found")
        except ValueError:
            print("Error: Invalid ID. Please provide a number")

    elif command == "help":
        print_usage()
    else:
        print(f"Unknown command: {command}")
        print_usage()

if __name__ == "__main__":
    main()

