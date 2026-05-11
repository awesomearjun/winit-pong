use winit::event_loop::{self, EventLoop};

mod app;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = app::App::default();

    let _ = event_loop.run_app(&mut app);
}
