use crate::input::InputBuffer;
use crate::renderer::Renderer;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct WindowManager {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl WindowManager {
    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("my_terminal"))
                .unwrap(),
        );

        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));

        self.renderer = Some(renderer);
        self.window = Some(window);
    }

    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    pub fn render(&mut self, input_buffer: &InputBuffer) {
        if let Some(renderer) = &mut self.renderer {
            match renderer.render(input_buffer.get_buffer()) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => {
                    if let Some(window) = &self.window {
                        let size = window.inner_size();
                        renderer.resize(size);
                    }
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    eprintln!("Mémoire GPU insuffisante");
                }
                Err(e) => {
                    eprintln!("Erreur de rendu: {:?}", e);
                }
            }
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(new_size);
        }
    }
}
