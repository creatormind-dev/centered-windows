#![forbid(unsafe_code)]

use std::sync::Arc;
use pollster::FutureExt;
use winit::{
	application::ApplicationHandler,
	dpi::{LogicalPosition, LogicalSize, PhysicalSize},
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	monitor::MonitorHandle,
	window::{Window, WindowId},
};


pub struct StateOverlay<'a> {
	state: Option<State<'a>>,
}


impl<'a> StateOverlay<'a> {
	pub fn new() -> Self {
		Self {
			state: None,
		}
	}
}

impl<'a> ApplicationHandler for StateOverlay<'a> {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window_attributes = Window::default_attributes()
			.with_title("Centered Windows")
			.with_decorations(false)
			.with_resizable(false)
			.with_transparent(true);

		let window = event_loop.create_window(window_attributes).unwrap();
		let (position, size) = Self::calculate_display_area(&window);

		let _ = window.request_inner_size(size);
		window.set_outer_position(position);

		println!("Overlay with size {}x{} positioned at ({}, {})", size.width, size.height, position.x, position.y);

		self.state = Some(State::new(window));
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
		let window = self.state.as_ref().unwrap().window();

		if window.id() == window_id {
			match event {
				WindowEvent::CloseRequested => {
					event_loop.exit();
				},
				WindowEvent::Resized(physical_size) => {
					self.state.as_mut().unwrap().resize(physical_size);
				},
				WindowEvent::RedrawRequested => {
					self.state.as_mut().unwrap().render().unwrap();
				},
				_ => ()
			}
		}
	}

	fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
		let window = self.state.as_ref().unwrap().window();
		
		window.request_redraw();
	}
}

impl<'a> StateOverlay<'a> {
	/**
	This function calculates the display area for the overlay window to be rendered.

	It will return the position and size of the overlay window.
	*/
	fn calculate_display_area(window: &Window) -> (LogicalPosition<i32>, LogicalSize<u32>) {
		let available_monitors: Vec<MonitorHandle> = window.available_monitors().collect();

		// For each monitor detected, an accumulator is used to add each monitor's width and height
		// such that only the required space to cover all monitors is calculated.
		// And so, the overlay window's position should be where it can cover all monitors.

		// TODO: Change the accumulator to return only the NECESSARY space.
		available_monitors
			.iter()
			.fold((LogicalPosition::default(), LogicalSize::default()), |acc: (LogicalPosition<i32>, LogicalSize<u32>), monitor| {
				let monitor_position = monitor.position();
				let monitor_size = monitor.size();

				(
					LogicalPosition::new(acc.0.x.min(monitor_position.x), acc.0.y.min(monitor_position.y)),
					LogicalSize::new(acc.1.width + monitor_size.width, acc.1.height + monitor_size.height)
				)
			})
	}
}


struct State<'a> {
	surface: wgpu::Surface<'a>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,

	size: PhysicalSize<u32>,
	window: Arc<Window>,
}

impl<'a> State<'a> {
	fn new(window: Window) -> Self {
		let window_arc = Arc::new(window);
		let size = window_arc.inner_size();
		let instance = Self::create_gpu_instance();
		let surface = instance.create_surface(window_arc.clone()).unwrap();
		let adapter = Self::create_adapter(instance, &surface);
		let (device, queue) = Self::create_device(&adapter);
		let surface_capabilities = surface.get_capabilities(&adapter);
		let config = Self::create_surface_config(size, surface_capabilities);

		surface.configure(&device, &config);

		Self {
			surface,
			device,
			queue,
			config,
			size,
			window: window_arc,
		}
	}

	fn create_gpu_instance() -> wgpu::Instance {
		wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			..Default::default()
		})
	}

	fn create_adapter(instance: wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
		instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			}
		).block_on().unwrap()
	}

	fn create_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
		adapter.request_device(
			&wgpu::DeviceDescriptor {
				required_features: wgpu::Features::empty(),
				required_limits: wgpu::Limits::default(),
				memory_hints: wgpu::MemoryHints::default(),
				label: None,
			},
			None
		).block_on().unwrap()
	}

	fn create_surface_config(size: PhysicalSize<u32>, capabilities: wgpu::SurfaceCapabilities) -> wgpu::SurfaceConfiguration {
		let surface_format = capabilities.formats.iter()
			.find(|f| f.is_srgb())
			.copied()
			.unwrap_or(capabilities.formats[0]);

		wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::AutoNoVsync,
			alpha_mode: capabilities.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
		}
	}

	pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);
    }

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture().unwrap();
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.0,
							g: 0.0,
							b: 0.0,
							a: 0.4,
						}),
						store: wgpu::StoreOp::Store,
					}
				})],
				depth_stencil_attachment: None,
				occlusion_query_set: None,
				timestamp_writes: None,
			});
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}

	fn window(&self) -> &Window {
		&self.window
	}
}
