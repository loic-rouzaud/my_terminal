pub mod cd;
pub mod clear;
pub mod echo;
pub mod list;
pub mod pwd;
pub mod remove;

use crate::input::{ColoredText, InputBuffer};

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

    let mut prompt_parts = Vec::new();
    prompt_parts.push(ColoredText::colored(
        format!("{} - $ [", user),
        [0.7, 0.7, 0.7, 1.0],
    ));
    prompt_parts.push(ColoredText::colored(
        display_path.clone(),
        [0.5, 0.8, 1.0, 1.0],
    ));
    prompt_parts.push(ColoredText::colored(
        "] <()> ".to_string(),
        [0.7, 0.7, 0.7, 1.0],
    ));
    prompt_parts.push(ColoredText::colored(
        input.to_string(),
        [1.0, 1.0, 1.0, 1.0],
    ));

    buffer.push_colored_line(prompt_parts);

    match parts[0] {
        "clear" => clear::execute(buffer),
        "echo" => echo::execute(buffer, &parts[1..]),
        "ls" => list::execute(buffer, &parts[1..]),
        "cd" => cd::execute(buffer, &parts[1..]),
        "rm" => remove::execute(buffer, &parts[1..]),
        "pwd" => pwd::execute(buffer),
        _ => {
            let mut error_line = Vec::new();
            error_line.push(ColoredText::colored(
                format!("{} : Command not found", input),
                [1.0, 0.3, 0.3, 1.0],
            ));
            buffer.push_colored_line(error_line);
            buffer.push_plain_line("");
        }
    }
}
