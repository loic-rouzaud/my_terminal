pub mod clear;
pub mod echo;

use crate::input::InputBuffer;

pub fn run_command(input: &str, buffer: &mut InputBuffer) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "clear" => clear::execute(buffer),
        "echo" => echo::execute(&parts[1..], buffer),
        _ => buffer.add_text("Command not found"),
    }
}
