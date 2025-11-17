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
                println!("Fermeture de la fenêtre");
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                // Gérer l'input clavier
                self.input_buffer.handle_key_event(key_event);

                // Optionnel : mettre à jour le titre
                self.window_manager.update_title(&self.input_buffer);

                // Demander un redraw après chaque input
                self.window_manager.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                // Dessiner le texte dans la fenêtre
                self.window_manager.render(&self.input_buffer);
            }

            WindowEvent::Resized(physical_size) => {
                // Gérer le redimensionnement
                self.window_manager.resize(physical_size);
            }

            _ => (),
        }
    }
}
