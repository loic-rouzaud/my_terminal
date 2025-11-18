use crate::input::{ColoredText, InputBuffer};
use std::fs;
use std::os::unix::fs::PermissionsExt;
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
                    _ => buffer.push_plain_line(&format!("Option inconnue : -{}", c)),
                }
            }
        } else {
            dir = arg;
        }
    }

    let entries: Vec<_> = match fs::read_dir(dir) {
        Ok(e) => e.flatten().collect(),
        Err(_) => {
            buffer.push_plain_line(&format!("Erreur : le dossier '{}' n'existe pas.", dir));
            buffer.push_plain_line("");
            return;
        }
    };

    let mut entries: Vec<_> = entries.into_iter().collect();
    entries.sort_by_key(|e| e.file_name());

    let single_line = !long_format && entries.len() <= 8;

    if single_line {
        let mut parts: Vec<ColoredText> = Vec::new();
        for entry in entries {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if !show_all && file_name.starts_with('.') {
                continue;
            }
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let (display, color) = if metadata.is_dir() {
                (format!("{}/", file_name), [0.4, 0.5, 1.0, 1.0])
            } else if metadata.permissions().mode() & 0o111 != 0 {
                (format!("{}*", file_name), [1.0, 0.3, 0.3, 1.0])
            } else {
                (file_name.clone(), [1.0, 1.0, 1.0, 1.0])
            };
            parts.push(ColoredText::colored(format!("{} ", display), color));
        }
        if !parts.is_empty() {
            buffer.push_colored_line(parts);
        } else {
            buffer.push_plain_line("");
        }
    } else {
        for entry in entries {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if !show_all && file_name.starts_with('.') {
                continue;
            }
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            let is_dir = metadata.is_dir();
            let is_exec = !is_dir && (metadata.permissions().mode() & 0o111 != 0);

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

                let mut parts: Vec<ColoredText> = Vec::new();
                parts.push(ColoredText::plain(format!("{} ", perm_string)));
                parts.push(ColoredText::plain(format!("{:>8} ", size)));
                parts.push(ColoredText::plain(format!("{} ", datetime)));
                let name_part = if is_dir {
                    ColoredText::colored(format!("{}/", file_name), [0.4, 0.5, 1.0, 1.0])
                } else if is_exec {
                    ColoredText::colored(file_name.clone(), [1.0, 0.3, 0.3, 1.0])
                } else {
                    ColoredText::plain(file_name.clone())
                };
                parts.push(name_part);
                buffer.push_colored_line(parts);
            } else {
                let part = if is_dir {
                    ColoredText::colored(format!("{}/", file_name), [0.4, 0.5, 1.0, 1.0])
                } else if is_exec {
                    ColoredText::colored(file_name.clone(), [1.0, 0.3, 0.3, 1.0])
                } else {
                    ColoredText::plain(file_name.clone())
                };
                buffer.push_colored_line(vec![part]);
            }
        }
    }

    buffer.push_plain_line("");
}
