use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

#[derive(Default)]
pub struct InputBuffer {
    buffer: String,
    history: Vec<String>,
}

impl InputBuffer {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.state == ElementState::Pressed {
            match key_event.logical_key.as_ref() {
                Key::Named(NamedKey::Enter) => {
                    self.on_enter();
                }
                Key::Named(NamedKey::Backspace) => {
                    self.delete_char();
                }
                Key::Named(NamedKey::Space) => {
                    self.add_space();
                }
                Key::Character(c) => {
                    self.add_text(c);
                }
                _ => {}
            }
        }
    }

    fn on_enter(&mut self) {
        self.history.push(self.buffer.clone()); // Ajout de l'historique pour tester
        println!("last : {}", self.buffer);

        self.buffer.clear();
    }

    fn delete_char(&mut self) {
        self.buffer.pop();
    }

    fn add_space(&mut self) {
        self.buffer.push(' ');
    }

    fn add_text(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}
