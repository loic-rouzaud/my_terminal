use crate::input::InputBuffer;
use crate::window::WindowManager;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

#[derive(Default)]
pub struct App {
    window_manager: WindowManager,
    input_buffer: InputBuffer,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window_manager.create_window(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("window closed");
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                self.input_buffer.handle_key_event(key_event);

                self.window_manager.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                self.window_manager.render(&self.input_buffer);
            }

            WindowEvent::Resized(physical_size) => {
                self.window_manager.resize(physical_size);
            }

            _ => (),
        }
    }
}
