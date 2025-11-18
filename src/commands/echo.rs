use crate::input::InputBuffer;

pub fn execute(buffer: &mut InputBuffer, args: &[&str]) {
    let text = args.join(" ");
    buffer.history.push(text);
    buffer.history.push(String::new());
}
