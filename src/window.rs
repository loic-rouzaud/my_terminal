use crate::input::InputBuffer;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

#[derive(Default)]
pub struct WindowManager {
    window: Option<Window>,
}

impl WindowManager {
    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes().with_title("my_terminal"))
                .unwrap(),
        );
    }

    pub fn update_title(&self, input_buffer: &InputBuffer) {
        if let Some(window) = &self.window {
            let title = if input_buffer.is_empty() {
                "my_terminal".to_string()
            } else {
                format!("Input: {}", input_buffer.get_buffer())
            };
            window.set_title(&title);
        }
    }

    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
