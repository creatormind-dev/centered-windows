#![forbid(unsafe_code)]

mod overlay;

use overlay::Overlay;

use winit::{
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop}
};


fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut overlay = Overlay::default();

    event_loop.run_app(&mut overlay)
}
