use crate::commands::run_command;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

#[derive(Default)]
pub struct InputBuffer {
    buffer: String,
    pub history: Vec<String>,
    scroll_offset: i32,
}

impl InputBuffer {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.state == ElementState::Pressed {
            match key_event.logical_key.as_ref() {
                Key::Named(NamedKey::Enter) => {
                    let cmd = self.buffer.clone();
                    run_command(&cmd, self);
                    self.history.push(self.buffer.clone());
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
    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        self.scroll_offset = 0;
    }
}
