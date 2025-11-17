use crate::input::InputBuffer;

pub fn execute(args: &[&str], buffer: &mut InputBuffer) {
    let text = args.join(" ");
    buffer.history.push(text);
}
