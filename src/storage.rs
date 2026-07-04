use std::fs;
use std::path::PathBuf;

use crate::note::Note;

const NOTES_FILE: &str = "notes.json";

fn get_data_dir() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("note-app");
    fs::create_dir_all(&dir).ok();
    dir
}

fn get_notes_path() -> PathBuf {
    get_data_dir().join(NOTES_FILE)
}

pub fn load_notes() -> Vec<Note> {
    let path = get_notes_path();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_notes(notes: &[Note]) -> Result<(), String> {
    let path = get_notes_path();
    let data = serde_json::to_string_pretty(notes).map_err(|e| e.to_string())?;
    fs::write(&path, data).map_err(|e| e.to_string())?;
    Ok(())
}
