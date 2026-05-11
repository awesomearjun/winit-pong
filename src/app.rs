use winit::{
    application::ApplicationHandler,
    event::*,
    keyboard::{Key, NamedKey},
    window::Window,
};

pub struct App {
    window: Option<Box<Window>>,
}

impl Default for App {
    fn default() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Box::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        ));
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if event.logical_key == Key::Named(NamedKey::Escape) {
                        event_loop.exit();
                    }
                }
            }

            _ => {}
        }
    }
}
