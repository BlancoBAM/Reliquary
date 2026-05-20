/// undoops.rs — In-memory undo stack for Reliquary file operations.
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, sync::{Arc, Mutex}};
use tauri::Window;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "kind")]
pub enum UndoEntry {
    Move { originals: Vec<String>, dst: String },
    Copy { copies: Vec<String> },
    Rename { from: String, to: String },
    Create { path: String, is_dir: bool },
    Delete { description: String },
}

pub struct UndoStack {
    entries: Mutex<Vec<UndoEntry>>,
}

impl Default for UndoStack {
    fn default() -> Self { Self { entries: Mutex::new(Vec::new()) } }
}

impl UndoStack {
    pub fn push(&self, entry: UndoEntry) {
        let mut g = self.entries.lock().unwrap();
        if g.len() >= 50 { g.remove(0); }
        g.push(entry);
    }
    pub fn pop(&self) -> Option<UndoEntry> { self.entries.lock().unwrap().pop() }
    pub fn len(&self) -> usize { self.entries.lock().unwrap().len() }
}

pub async fn apply_undo(stack: &UndoStack) -> Result<String> {
    let entry = stack.pop().ok_or_else(|| anyhow!("Nothing to undo"))?;
    match &entry {
        UndoEntry::Move { originals, dst } => {
            let dst_path = Path::new(dst);
            for src_orig in originals {
                let filename = Path::new(src_orig).file_name()
                    .ok_or_else(|| anyhow!("Invalid path: {}", src_orig))?;
                let current = dst_path.join(filename);
                let orig_parent = Path::new(src_orig).parent()
                    .ok_or_else(|| anyhow!("No parent for {}", src_orig))?;
                fs::create_dir_all(orig_parent)?;
                let opts = fs_extra::dir::CopyOptions::new();
                fs_extra::move_items(&[&current], orig_parent, &opts)
                    .with_context(|| format!("move {} back", current.display()))?;
            }
            Ok(format!("Undid move of {} item(s)", originals.len()))
        }
        UndoEntry::Copy { copies } => {
            for p in copies {
                let path = Path::new(p);
                if path.is_dir() { fs::remove_dir_all(path)?; } else { fs::remove_file(path)?; }
            }
            Ok(format!("Undid copy of {} item(s)", copies.len()))
        }
        UndoEntry::Rename { from, to } => {
            fs::rename(to, from).with_context(|| format!("rename {} -> {}", to, from))?;
            Ok(format!("Undid rename: {} -> {}", to, from))
        }
        UndoEntry::Create { path, is_dir } => {
            let p = Path::new(path);
            if *is_dir { fs::remove_dir_all(p)?; } else { fs::remove_file(p)?; }
            Ok(format!("Undid creation of {}", path))
        }
        UndoEntry::Delete { description } => {
            Err(anyhow!("Cannot undo permanent delete: {}", description))
        }
    }
}

#[tauri::command]
pub async fn undo_last_op(
    state: tauri::State<'_, Arc<UndoStack>>,
    _window: Window,
) -> Result<String, String> {
    apply_undo(&state).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn undo_stack_len(
    state: tauri::State<'_, Arc<UndoStack>>,
) -> Result<usize, String> {
    Ok(state.len())
}
