use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{Window, WindowAttributes}
};


#[derive(Default)]
pub struct Overlay {
	window: Option<Window>
}


impl ApplicationHandler for Overlay {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(WindowAttributes::default()).unwrap());
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping...");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => ()
        };
    }
}
