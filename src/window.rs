use crate::input::InputBuffer;
use crate::renderer::Renderer;
use std::sync::Arc;
use winit::event::MouseScrollDelta;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct WindowManager {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    scroll_offset: f32,
    max_scroll: f32,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self {
            window: None,
            renderer: None,
            scroll_offset: 0.0,
            max_scroll: 10000.0, // <- horrible mais je vois pas comment faire autrement pour l'instant
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
        window.set_min_inner_size(Some(winit::dpi::PhysicalSize::new(300, 200))); // bloque la taille de la window

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
            match renderer.render(input_buffer.get_buffer(), input_buffer.get_history()) {
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

    pub fn handle_scroll(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                self.scroll_offset -= y * 20.0;
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.scroll_offset -= pos.y as f32;
            }
        }

        if self.scroll_offset < 0.0 {
            self.scroll_offset = 0.0;
        }
        if self.scroll_offset > self.max_scroll {
            self.scroll_offset = self.max_scroll;
        }
    }
}
