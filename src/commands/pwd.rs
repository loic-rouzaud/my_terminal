use crate::input::{ColoredText, InputBuffer};
use std::env;

pub fn execute(buffer: &mut InputBuffer) {
    let line = match env::current_dir() {
        Ok(path) => {
            let text = path.to_string_lossy().to_string();
            vec![ColoredText::colored(text, [0.8, 0.8, 1.0, 1.0])]
        }
        Err(e) => {
            vec![ColoredText::colored(
                format!("pwd: error while executing: {}", e),
                [1.0, 0.5, 0.5, 1.0],
            )]
        }
    };

    buffer.push_colored_line(line);
    buffer.push_plain_line("");
}
