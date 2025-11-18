use crate::input::InputBuffer;
use std::env;

pub fn execute(buffer: &mut InputBuffer) {
    match env::current_dir() {
        Ok(path) => {
            buffer.history.push(path.to_string_lossy().to_string());
        }
        Err(e) => {
            buffer
                .history
                .push(format!("pwd: error while executing: {}", e));
        }
    }

    buffer.history.push(String::new());
}
