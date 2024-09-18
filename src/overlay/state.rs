use pollster::FutureExt;
use wgpu::util::DeviceExt;

use std::sync::Arc;
use winit::{
	dpi::PhysicalSize,
	window::Window,
};


/**
Defines the input model for a Vertex in the shader.
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
	position: [f32; 2],
}

// TODO: Remove hardcoded vertices and indices.

const VERTICES: &[Vertex] = &[
	Vertex { position: [0.5, 0.75] },
	Vertex { position: [-0.5, 0.75] },
	Vertex { position: [-0.5, -0.75] },
	Vertex { position: [0.5, -0.75] },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

impl Vertex {
	fn desc() -> wgpu::VertexBufferLayout<'static> {
		use std::mem;

		wgpu::VertexBufferLayout {
			array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttribute {
					// No offset because this is the first (and only) attribute.
					offset: 0,
					// Corresponds to @location(0) in the shader code.
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2,
				}
			],
		}
	}
}


pub struct State<'a> {
	pub size: PhysicalSize<u32>,

	surface: wgpu::Surface<'a>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	render_pipeline: wgpu::RenderPipeline,

	window: Arc<Window>,
}


impl<'a> State<'a> {
	pub fn new(window: Window) -> Self {
		let window_arc = Arc::new(window);
		let size = window_arc.inner_size();

		let instance = Self::create_instance();
		let surface = instance.create_surface(window_arc.clone()).unwrap();
		let adapter = Self::create_adapter(instance, &surface);
		let (device, queue) = Self::create_device(&adapter);
		let surface_capabilities = surface.get_capabilities(&adapter);
		let config = Self::create_surface_config(size, surface_capabilities);

		surface.configure(&device, &config);

		let render_pipeline = Self::create_render_pipeline(&device, &config);

		Self {
			size,

			surface,
			device,
			queue,
			config,
			render_pipeline,

			window: window_arc,
		}
	}

	fn create_instance() -> wgpu::Instance {
		wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			..Default::default()
		})
	}

	fn create_adapter(instance: wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
		instance
			.request_adapter(
				&wgpu::RequestAdapterOptions {
					power_preference: wgpu::PowerPreference::default(),
					compatible_surface: Some(&surface),
					force_fallback_adapter: false,
				}
			)
			.block_on()
			.unwrap()
	}

	fn create_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
		adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					required_features: wgpu::Features::empty(),
					required_limits: wgpu::Limits::default(),
					memory_hints: wgpu::MemoryHints::default(),
					label: None,
				},
				None
			)
			.block_on()
			.unwrap()
	}

	fn create_surface_config(size: PhysicalSize<u32>, capabilities: wgpu::SurfaceCapabilities) -> wgpu::SurfaceConfiguration {
		// Looks for a sRGB compatible surface.
		let surface_format = capabilities.formats
			.iter()
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

	fn create_render_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
		});

		let layout = device.create_pipeline_layout(
			&wgpu::PipelineLayoutDescriptor {
				label: Some("Render Pipeline Layout"),
				bind_group_layouts: &[],
				push_constant_ranges: &[],
			},
		);

		device.create_render_pipeline(
			&wgpu::RenderPipelineDescriptor {
				label: Some("Render Pipeline"),
				layout: Some(&layout),
				vertex: wgpu::VertexState {
					module: &shader,
					entry_point: "vs_main",
					compilation_options: wgpu::PipelineCompilationOptions::default(),
					buffers: &[
						Vertex::desc(),
					],
				},
				fragment: Some(wgpu::FragmentState {
					module: &shader,
					entry_point: "fs_main",
					compilation_options: wgpu::PipelineCompilationOptions::default(),
					targets: &[
						Some(wgpu::ColorTargetState {
							format: config.format,
							blend: Some(wgpu::BlendState::REPLACE),
							write_mask: wgpu::ColorWrites::ALL,
						}),
					],
				}),
				primitive: wgpu::PrimitiveState {
					topology: wgpu::PrimitiveTopology::TriangleList,
					strip_index_format: None,
					front_face: wgpu::FrontFace::Ccw,
					cull_mode: Some(wgpu::Face::Back),
					polygon_mode: wgpu::PolygonMode::Fill,
					unclipped_depth: false,
					conservative: false,
				},
				depth_stencil: None,
				multisample: wgpu::MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				},
				multiview: None,
				cache: None,
			},
		)
	}

	fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
		device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: bytemuck::cast_slice(vertices),
				usage: wgpu::BufferUsages::VERTEX,
			},
		)
	}

	fn create_index_buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {
		device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Index Buffer"),
				contents: bytemuck::cast_slice(indices),
				usage: wgpu::BufferUsages::INDEX,
			},
		)
	}
}


impl<'a> State<'a> {
	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);
    }

	pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let mut render_pass = encoder.begin_render_pass(
				&wgpu::RenderPassDescriptor {
					label: Some("Render Pass"),
					color_attachments: &[
						Some(wgpu::RenderPassColorAttachment {
							view: &view,
							resolve_target: None,
							ops: wgpu::Operations {
								load: wgpu::LoadOp::Clear(wgpu::Color {
									r: 0.0,
									g: 0.0,
									b: 0.0,
									a: 1.0,
								}),
								store: wgpu::StoreOp::Store,
							},
						},
					)],
					depth_stencil_attachment: None,
					occlusion_query_set: None,
					timestamp_writes: None,
				},
			);

			let vertex_buffer = Self::create_vertex_buffer(&self.device, VERTICES);
			let index_buffer = Self::create_index_buffer(&self.device, INDICES);

			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
			render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
			render_pass.draw_indexed(0..INDICES.len() as _, 0, 0..1);
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}