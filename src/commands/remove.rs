use crate::input::InputBuffer;
use std::fs;
use std::path::Path;

pub fn execute(buffer: &mut InputBuffer, args: &[&str]) {
    let mut recursive = false;
    let mut force = false;
    let mut targets: Vec<&str> = vec![];

    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'r' => recursive = true,
                    'f' => force = true,
                    _ => buffer.history.push(format!("Option inconnue : -{}", c)),
                }
            }
        } else {
            targets.push(arg);
        }
    }

    if targets.is_empty() {
        buffer
            .history
            .push("rm: aucun fichier ou dossier spécifié".into());
        buffer.history.push(String::new());
        return;
    }

    for target in targets {
        let path = Path::new(target);

        if !path.exists() {
            if !force {
                buffer
                    .history
                    .push(format!("rm: {}: fichier ou dossier inexistant", target));
            }
            continue;
        }

        if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    buffer
                        .history
                        .push(format!("rm: impossible de supprimer {}: {}", target, e));
                }
            } else {
                buffer.history.push(format!(
                    "rm: {} est un dossier (utiliser -r pour récursif)",
                    target
                ));
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                buffer
                    .history
                    .push(format!("rm: impossible de supprimer {}: {}", target, e));
            }
        }
    }

    buffer.history.push(String::new());
}
