use crate::commands::run_command;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

#[derive(Clone, Debug)]
pub struct ColoredText {
    pub text: String,
    pub color: [f32; 4],
}

impl ColoredText {
    pub fn plain<S: Into<String>>(s: S) -> Self {
        Self {
            text: s.into(),
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn colored<S: Into<String>>(s: S, color: [f32; 4]) -> Self {
        Self {
            text: s.into(),
            color,
        }
    }
}

#[derive(Default)]
pub struct InputBuffer {
    buffer: String,
    pub history: Vec<Vec<ColoredText>>,
    pub scroll_offset: i32,
}

impl InputBuffer {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.state == ElementState::Pressed {
            match key_event.logical_key.as_ref() {
                Key::Named(NamedKey::Enter) => {
                    let cmd = self.buffer.clone();
                    if cmd.trim() == "clear" {
                        self.clear_history();
                    } else {
                        run_command(&cmd, self);
                    }
                    self.buffer.clear();
                    self.scroll_offset = 0;
                }
                Key::Named(NamedKey::Backspace) => self.delete_char(),
                Key::Named(NamedKey::Space) => self.add_space(),
                Key::Character(c) => self.add_text(c),
                _ => {}
            }
        }
    }

    fn delete_char(&mut self) {
        self.buffer.pop();
    }

    fn add_space(&mut self) {
        self.buffer.push(' ');
    }

    pub fn add_text(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    pub fn get_history(&self) -> &Vec<Vec<ColoredText>> {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        self.scroll_offset = 0;
    }

    pub fn push_plain_line(&mut self, s: &str) {
        self.history.push(vec![ColoredText::plain(s)]);
    }

    pub fn push_colored_line(&mut self, parts: Vec<ColoredText>) {
        self.history.push(parts);
    }
}
