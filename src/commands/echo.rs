use crate::input::{ColoredText, InputBuffer};

pub fn execute(buffer: &mut InputBuffer, args: &[&str]) {
    let text = args.join(" ");

    let line = vec![ColoredText::colored(text, [1.0, 1.0, 1.0, 1.0])];

    buffer.push_colored_line(line);
    buffer.push_plain_line("");
}
