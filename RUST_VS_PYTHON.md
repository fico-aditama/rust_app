# Perbandingan Rust vs Python

Dokumen ini membandingkan implementasi aplikasi Note Manager yang sama dalam Rust dan Python.

## 📊 Ringkasan Perbedaan

| Aspek | Python | Rust |
|-------|--------|------|
| **Syntax** | Lebih sederhana, lebih readable | Lebih verbose, lebih eksplisit |
| **Type System** | Dynamic typing (opsional type hints) | Static typing (wajib) |
| **Memory Management** | Garbage collector (otomatis) | Ownership system (manual) |
| **Error Handling** | Try/except (exceptions) | Result/Option types |
| **Performance** | Lambat (interpreted) | Sangat cepat (compiled) |
| **Compile Time** | Tidak perlu compile | Perlu compile dulu |
| **Null Safety** | None (runtime errors) | Option<T> (compile-time safety) |
| **Concurrency** | GIL (Global Interpreter Lock) | Zero-cost abstractions |

---

## 🔍 Perbandingan Kode Sisi-ke-Sisi

### 1. Definisi Struct/Class

**Python:**
```python
class Note:
    def __init__(self, id: int, content: str, created_at: str):
        self.id = id
        self.content = content
        self.created_at = created_at
```

**Rust:**
```rust
struct Note {
    id: usize,
    content: String,
    created_at: String,
}
```

**Perbedaan:**
- Python: Lebih verbose dengan `__init__`, tapi lebih fleksibel
- Rust: Lebih ringkas, tapi harus eksplisit tentang tipe data

---

### 2. Method Implementation

**Python:**
```python
def add(self, content: str):
    note = Note(
        id=self.next_id,
        content=content,
        created_at=datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    )
    self.notes.append(note)
    self.next_id += 1
```

**Rust:**
```rust
fn add(&mut self, content: String) {
    let note = Note {
        id: self.next_id,
        content,
        created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    self.notes.push(note);
    self.next_id += 1;
}
```

**Perbedaan:**
- Python: `self` parameter eksplisit, tidak perlu `&mut`
- Rust: `&mut self` menunjukkan bahwa method mengubah state (mutable reference)
- Rust: `content` bisa di-shorthand jika nama field sama dengan variable

---

### 3. Error Handling

**Python:**
```python
def load_notes() -> Notes:
    try:
        with open("notes.json", "r") as f:
            data = json.load(f)
            return Notes.from_dict(data)
    except (FileNotFoundError, json.JSONDecodeError):
        return Notes()
```

**Rust:**
```rust
fn load_notes() -> Notes {
    match fs::read_to_string("notes.json") {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| Notes::new())
        }
        Err(_) => Notes::new(),
    }
}
```

**Perbedaan:**
- Python: Try/except - exceptions bisa terjadi di mana saja
- Rust: Result<T, E> - error handling eksplisit, compiler memaksa handle error
- Rust: Lebih aman karena tidak bisa "lupa" handle error

---

### 4. File I/O

**Python:**
```python
def save_notes(notes: Notes) -> bool:
    try:
        with open("notes.json", "w") as f:
            json.dump(notes.to_dict(), f, indent=2)
        return True
    except Exception:
        return False
```

**Rust:**
```rust
fn save_notes(notes: &Notes) -> io::Result<()> {
    let json = serde_json::to_string_pretty(notes)?;
    fs::write("notes.json", json)?;
    Ok(())
}
```

**Perbedaan:**
- Python: `with` statement untuk auto-close file
- Rust: `?` operator untuk propagate error (lebih ringkas)
- Rust: `&Notes` adalah reference (borrowing), tidak transfer ownership

---

### 5. Pattern Matching

**Python:**
```python
if command == "add":
    # ...
elif command == "list":
    # ...
elif command == "delete":
    # ...
else:
    # ...
```

**Rust:**
```rust
match args[1].as_str() {
    "add" => { /* ... */ }
    "list" => { /* ... */ }
    "delete" => { /* ... */ }
    _ => { /* ... */ }
}
```

**Perbedaan:**
- Python: If/elif chain
- Rust: `match` expression - compiler memastikan semua case ter-handle
- Rust: Exhaustive checking (tidak bisa lupa handle case)

---

## 🚀 Performance Comparison

### Compile Time
- **Python**: Tidak perlu compile, langsung run
- **Rust**: Perlu compile dulu (`cargo build`), tapi hasilnya sangat cepat

### Runtime Performance
- **Python**: Interpreted, lebih lambat
- **Rust**: Compiled to native code, sangat cepat (bisa 10-100x lebih cepat)

### Memory Usage
- **Python**: Lebih banyak memory (GC overhead)
- **Rust**: Lebih efisien (zero-cost abstractions)

---

## 🛡️ Safety Features

### Null Safety

**Python:**
```python
value = some_dict.get("key")  # Bisa return None
if value is None:  # Harus manual check
    # handle
```

**Rust:**
```rust
let value: Option<String> = some_map.get("key");
match value {
    Some(v) => { /* ... */ }
    None => { /* ... */ }
}
// Compiler memaksa handle None case!
```

### Memory Safety

**Python:**
```python
# Bisa terjadi runtime error
items = [1, 2, 3]
print(items[10])  # IndexError di runtime
```

**Rust:**
```rust
// Compiler error jika index out of bounds
let items = vec![1, 2, 3];
// items[10];  // Compile error!
items.get(10);  // Return Option, aman
```

---

## 📝 Kapan Pakai Python vs Rust?

### Gunakan Python jika:
- ✅ Prototyping cepat
- ✅ Scripting dan automation
- ✅ Data science / ML
- ✅ Web development (Django, Flask)
- ✅ Tidak perlu performa ekstrem
- ✅ Team lebih familiar dengan Python

### Gunakan Rust jika:
- ✅ Sistem programming
- ✅ Performance critical (game engines, databases)
- ✅ Concurrency tinggi
- ✅ Memory safety penting
- ✅ Embedded systems
- ✅ WebAssembly
- ✅ Ingin zero-cost abstractions

---

## 🎯 Kesimpulan

**Python:**
- Lebih mudah dipelajari
- Development lebih cepat
- Syntax lebih readable
- Tapi lebih lambat dan kurang type-safe

**Rust:**
- Lebih sulit dipelajari (steep learning curve)
- Development lebih lambat (compile time + strictness)
- Tapi sangat cepat dan sangat aman
- Compiler membantu catch bugs sebelum runtime

**Keduanya punya tempat masing-masing!** Python untuk produktivitas, Rust untuk performa dan safety.

