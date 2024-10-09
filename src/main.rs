#![windows_subsystem = "windows"]

mod overlay;
mod data;

use overlay::OverlayApp;
use winit::event_loop::{ControlFlow, EventLoop};


fn main() {
    env_logger::builder()
        .format_target(false)
        .format_module_path(false)
        .init();

    let event_loop = EventLoop::new().unwrap();
    let mut app = OverlayApp::new();
    
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)
        .expect("OverlayApp Event Loop");
}
