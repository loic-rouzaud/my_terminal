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
    let home = std::env::var("HOME").unwrap_or("/".into());
    let current_dir =
        std::env::current_dir().unwrap_or_else(|_| std::path::Path::new(".").to_path_buf());

    let display_path = if let Ok(stripped) = current_dir.strip_prefix(&home) {
        let s = stripped.to_string_lossy();
        if s.is_empty() {
            "~".to_string()
        } else {
            format!("~{}", s)
        }
    } else {
        current_dir.to_string_lossy().to_string()
    };

    buffer
        .history
        .push(format!("{} - $ [{}] <()> {}", user, display_path, input));

    match parts[0] {
        "clear" => clear::execute(buffer),
        "echo" => echo::execute(buffer, &parts[1..]),
        "ls" => list::execute(buffer, &parts[1..]),
        "cd" => cd::execute(buffer, &parts[1..]),
        "pwd" => pwd::execute(buffer),
        _ => {
            buffer
                .history
                .push(format!("{} : Command not found", input));
            buffer.history.push(String::new());
        }
    }
}
