pub mod cd;
pub mod clear;
pub mod echo;
pub mod list;
pub mod pwd;

use crate::input::InputBuffer;

pub fn run_command(input: &str, buffer: &mut InputBuffer) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let user = std::env::var("USER").unwrap_or("user".into());
    buffer.history.push(format!("{} - $ {}", user, input));

    match parts[0] {
        "clear" => clear::execute(buffer),
        "echo" => echo::execute(buffer, &parts[1..]),
        "ls" => list::execute(buffer, &parts[1..]),
        "cd" => cd::execute(buffer, &parts[1..]),
        "pwd" => pwd::execute(buffer),
        _ => {
            buffer
                .history
                .push(format!("{} : Command not found", buffer.get_buffer()));
            buffer.history.push(String::new());
        }
    }
}
