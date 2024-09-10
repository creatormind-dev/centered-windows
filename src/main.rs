mod overlay;
mod app_window_info;

use overlay::Overlay;
use winit::event_loop::{ControlFlow, EventLoop};


fn main() {
    env_logger::init();

    for window in app_window_info::get_windows() {
        let title = window.title;
        let position = window.position;
        let size = window.size;

        println!("Detected window \"{}\" positioned at ({}, {}) with size {}x{}", title, position.x, position.y, size.width, size.height);
    }

    let event_loop = EventLoop::new().unwrap();
    
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut window_state = Overlay::new();
    let _ = event_loop.run_app(&mut window_state);
}
