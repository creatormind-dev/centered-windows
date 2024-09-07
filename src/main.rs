mod overlay;

use overlay::Overlay;
use winit::event_loop::{ControlFlow, EventLoop};


fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut window_state = Overlay::new();
    let _ = event_loop.run_app(&mut window_state);
}
