pub mod clear;
pub mod echo;
pub mod list;

use crate::input::InputBuffer;

pub fn run_command(input: &str, buffer: &mut InputBuffer) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "clear" => clear::execute(buffer),
        "echo" => echo::execute(&parts[1..], buffer),
        "ls" => list::execute(buffer),
        _ => {
            buffer
                .history
                .push(format!("{} : Command not found", buffer.get_buffer()));
            buffer.history.push(String::new());
        }
    }
}
