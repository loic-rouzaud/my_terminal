use crate::input::InputBuffer;
use std::env;
use std::path::Path;

pub fn execute(buffer: &mut InputBuffer, args: &[&str]) {
    let target = if args.is_empty() {
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                buffer
                    .history
                    .push("cd: impossible de déterminer HOME".into());
                buffer.history.push(String::new());
                return;
            }
        }
    } else {
        args[0].to_string()
    };

    let path = Path::new(&target);

    if let Err(e) = env::set_current_dir(&path) {
        buffer.history.push(format!("cd: {}: {}", target, e));
        buffer.history.push(String::new());
        return;
    }

    buffer.history.push(String::new());
}
