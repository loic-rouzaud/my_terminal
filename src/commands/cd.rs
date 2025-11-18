use crate::input::{ColoredText, InputBuffer};
use std::env;
use std::path::Path;

pub fn execute(buffer: &mut InputBuffer, args: &[&str]) {
    let target = if args.is_empty() {
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                buffer.push_colored_line(vec![ColoredText::colored(
                    "cd: no HOME directory".to_string(),
                    [1.0, 0.5, 0.5, 1.0],
                )]);
                buffer.push_plain_line("");
                return;
            }
        }
    } else {
        args[0].to_string()
    };

    let path = Path::new(&target);

    if let Err(e) = env::set_current_dir(&path) {
        buffer.push_colored_line(vec![ColoredText::colored(
            format!("cd: {}: {}", target, e),
            [1.0, 0.5, 0.5, 1.0],
        )]);
        buffer.push_plain_line("");
        return;
    }

    buffer.push_plain_line("");
}
