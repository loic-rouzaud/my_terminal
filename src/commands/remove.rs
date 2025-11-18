use crate::input::{ColoredText, InputBuffer};
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
                    _ => buffer.push_colored_line(vec![ColoredText::colored(
                        format!("Option inconnue : -{}", c),
                        [1.0, 0.5, 0.5, 1.0],
                    )]),
                }
            }
        } else {
            targets.push(arg);
        }
    }

    if targets.is_empty() {
        buffer.push_colored_line(vec![ColoredText::colored(
            "rm: aucun fichier ou dossier spécifié",
            [1.0, 0.5, 0.5, 1.0], // rouge
        )]);
        buffer.push_plain_line("");
        return;
    }

    for target in targets {
        let path = Path::new(target);

        if !path.exists() {
            if !force {
                buffer.push_colored_line(vec![ColoredText::colored(
                    format!("rm: {}: fichier ou dossier inexistant", target),
                    [1.0, 0.5, 0.5, 1.0], // rouge
                )]);
            }
            continue;
        }

        if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    buffer.push_colored_line(vec![ColoredText::colored(
                        format!("rm: impossible de supprimer {}: {}", target, e),
                        [1.0, 0.5, 0.5, 1.0], // rouge
                    )]);
                }
            } else {
                buffer.push_colored_line(vec![ColoredText::colored(
                    format!("rm: {} est un dossier (utiliser -r pour récursif)", target),
                    [1.0, 0.5, 0.5, 1.0], // rouge
                )]);
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                buffer.push_colored_line(vec![ColoredText::colored(
                    format!("rm: impossible de supprimer {}: {}", target, e),
                    [1.0, 0.5, 0.5, 1.0], // rouge
                )]);
            }
        }
    }

    buffer.push_plain_line("");
}
