#![forbid(unsafe_code)]

mod state;

use state::State;

use winit::{
	application::ApplicationHandler,
	dpi::{LogicalPosition, LogicalSize},
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	monitor::MonitorHandle,
	platform::windows::{CornerPreference, WindowAttributesExtWindows},
	window::{Window, WindowButtons, WindowId, WindowLevel}
};


pub struct OverlayApp<'a> {
	state: Option<State<'a>>,
}


impl<'a> OverlayApp<'a> {
	pub fn new() -> Self {
		Self {
			state: None,
		}
	}
}

impl<'a> ApplicationHandler for OverlayApp<'a> {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let (position, size) = Self::calculate_display_area(event_loop);
		let window_attributes = Window::default_attributes()
			.with_active(true)
			.with_content_protected(true)
			.with_corner_preference(CornerPreference::DoNotRound)
			.with_decorations(false)
			.with_drag_and_drop(false)
			.with_enabled_buttons(WindowButtons::empty())
			.with_inner_size(size)
			.with_position(position)
			.with_resizable(false)
			.with_skip_taskbar(true)
			.with_transparent(true)
			.with_window_level(WindowLevel::AlwaysOnTop);

		let window = event_loop.create_window(window_attributes).unwrap();

		self.state = Some(State::new(window));
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
		let state = self.state
			.as_mut()
			.unwrap();

		let window = state.window();

		if window.id() != window_id {
			return;
		}

		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			},
			WindowEvent::KeyboardInput { .. } => {
				event_loop.exit();
			},
			WindowEvent::Resized(physical_size) => {
				self.state.as_mut().unwrap().resize(physical_size);
			},
			WindowEvent::Focused(has_focus) => {
				if has_focus == false {
					event_loop.exit();
				}
			},
			WindowEvent::RedrawRequested => {
				self.state.as_mut().unwrap().render().unwrap();
			},
			_ => ()
		}
	}

	fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
		self.state
			.as_ref()
			.unwrap()
			.window()
			.request_redraw();
	}
}

impl<'a> OverlayApp<'a> {
	/**
	This function calculates the display area for the overlay window to be rendered.

	It will return the position and size of the overlay window.
	*/
	pub fn calculate_display_area(event_loop: &ActiveEventLoop) -> (LogicalPosition<i32>, LogicalSize<u32>) {
		let available_monitors: Vec<MonitorHandle> = event_loop.available_monitors().collect();

		// `min_x` and `max_y` track the position of where the overlay should be rendered. 
		// This is typically the top-left coordinate across all monitors.
		// `max_x` and `max_y` track the largest `x + width` and `y + height` values,
		// which determines where the bottom-right corner across all monitors is.

		let mut min_x = 0;
		let mut min_y = 0;
		let mut max_x = 0;
		let mut max_y = 0;

		for monitor in available_monitors.iter() {
			let size = monitor.size();
			let position = monitor.position();

			min_x = min_x.min(position.x);
			min_y = min_y.min(position.y);
			max_x = max_x.max(position.x + (size.width as i32));
			max_y = max_y.max(position.y + (size.height as i32));
		}

		// The difference between the top-left and bottom-right corner yields the resulting rect to be display.

		(LogicalPosition::new(min_x, min_y), LogicalSize::new((max_x - min_x) as u32, (max_y - min_y) as u32))
	}
}
