pub mod cd;
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
        "echo" => echo::execute(buffer, &parts[1..]),
        "ls" => list::execute(buffer, &parts[1..]),
        "cd" => cd::execute(buffer, &parts[1..]),
        _ => {
            buffer
                .history
                .push(format!("{} : Command not found", buffer.get_buffer()));
            buffer.history.push(String::new());
        }
    }
}
