use crate::input::InputBuffer;
use std::fs;

pub fn execute(buffer: &mut InputBuffer) {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let display = if file_type.is_dir() {
                        format!("{}/", name)
                    } else {
                        name
                    };
                    buffer.history.push(display);
                }
            }
            buffer.history.push(String::new()); // espace entre les commandes
        }
        Err(e) => buffer.history.push(format!("Erreur: {}", e)),
    }
}
