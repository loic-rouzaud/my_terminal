use crate::input::InputBuffer;
use std::fs;
use std::os::unix::fs::PermissionsExt; // affichage des permissions
use std::time::UNIX_EPOCH;

pub fn execute(buffer: &mut InputBuffer, args: &[&str]) {
    let mut show_all = false;
    let mut long_format = false;
    let mut dir = ".";

    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'l' => long_format = true,
                    _ => buffer.history.push(format!("Option inconnue : -{}", c)),
                }
            }
        } else {
            dir = arg;
        }
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => {
            buffer
                .history
                .push(format!("Erreur : le dossier '{}' n'existe pas.", dir));
            buffer.history.push(String::new());
            return;
        }
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();

        if !show_all && file_name.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let is_dir = metadata.is_dir();

        if long_format {
            let permissions = metadata.permissions().mode();
            let size = metadata.len();

            let datetime = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let type_char = if is_dir { 'd' } else { '-' };
            let perm_string = format!("{}{:o}", type_char, permissions & 0o777);

            buffer.history.push(format!(
                "{}  {:>8}  {}  {}",
                perm_string,
                size,
                datetime,
                if is_dir {
                    format!("{}/", file_name)
                } else {
                    file_name
                }
            ));
        } else {
            let display = if is_dir {
                format!("{}/", file_name)
            } else {
                file_name
            };
            buffer.history.push(display);
        }
    }

    buffer.history.push(String::new());
}
