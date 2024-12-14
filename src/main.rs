#![windows_subsystem = "windows"]

use centered_windows::*;
use winit::event_loop::{ControlFlow, EventLoop};


fn main() {
    init_logger().expect("Failed to initialize logger");

    let event_loop = EventLoop::new().unwrap();
    let mut app = OverlayApp::new();
    
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)
        .unwrap();
}
