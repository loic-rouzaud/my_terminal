use crate::input::InputBuffer;
use std::fs;

pub fn execute(buffer: &mut InputBuffer, args: &[&str]) {
    let dir = if args.is_empty() { "." } else { args[0] };

    match fs::read_dir(dir) {
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
            buffer.history.push(String::new());
        }

        Err(_) => {
            buffer
                .history
                .push(format!("Error : no such file or directory '{}'", dir));
            buffer.history.push(String::new());
        }
    }
}
