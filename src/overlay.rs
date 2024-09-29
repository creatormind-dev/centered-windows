use pollster::FutureExt;
use wgpu::util::DeviceExt;

use crate::data;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    monitor::MonitorHandle,
    window::{Window, WindowButtons, WindowId, WindowLevel}
};

#[cfg(target_os = "windows")]
use winit::platform::windows::{CornerPreference, WindowAttributesExtWindows};


pub struct OverlayApp<'a> {
    state: Option<State<'a>>,
    
    windows: Vec<data::WindowInfo>,
}

impl<'a> OverlayApp<'a> {
    pub fn new() -> Self {
        Self {
            state: None,
            windows: Vec::new(),
        }
    }
}

impl<'a> ApplicationHandler for OverlayApp<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let windows = data::get_windows().unwrap_or_else(|e| {
            panic!("Could not enumerate application windows: {e}");
        });
        
        let (position, size) = Self::calculate_display_area(&event_loop);

        let window_attributes = Window::default_attributes()
            .with_active(true)
            .with_content_protected(true)
            .with_decorations(false)
            .with_enabled_buttons(WindowButtons::empty())
            .with_inner_size(size)
            .with_position(position)
            .with_resizable(false)
            .with_transparent(true)
            .with_window_level(WindowLevel::AlwaysOnTop);

        // Specific window settings on Windows OS.
        #[cfg(target_os = "windows")]
        let window_attributes = window_attributes
            .with_corner_preference(CornerPreference::DoNotRound)
            .with_drag_and_drop(false)
            .with_skip_taskbar(true);

        let window = event_loop.create_window(window_attributes).unwrap();
        
        self.state = Some(State::new(window));
        self.windows = windows;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let state = self.state
            .as_mut()
            .unwrap();

        if state.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested |
            WindowEvent::KeyboardInput { .. } => {
                event_loop.exit();
            }

            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            }

            WindowEvent::Focused(has_focus) => {
                if has_focus == false {
                    event_loop.exit();
                }
            }
            
            WindowEvent::CursorMoved { position, .. } => {
                let clip = state.clip.unwrap_or(data::Rect::default());

                // If there is already a defined clip in the state, check if the same clip contains
                // the cursor coordinates to avoid fetching the same window rect multiple times.
                // If the clip is None the contains function will return false.
                
                if !clip.contains(position.x as i32, position.y as i32) {
                    let overlay_pos = state.window().inner_position().unwrap();
                    let overlay_size = state.size;
                    let overlay_rect = data::Rect::new(
                        overlay_pos.x,
                        overlay_pos.y,
                        overlay_size.width,
                        overlay_size.height,
                    );

                    let clip = self.windows
                        .iter()
                        .map(|w| data::Rect::adjust(w.rect(), overlay_rect))
                        .find(|r| r.contains(position.x as i32, position.y as i32));

                    state.clip = clip;
                }
            }

            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {}

                    Err(wgpu::SurfaceError::Lost |
                        wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }

                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        event_loop.exit()
                    }

                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Surface Timeout!");
                    }
                }
            }

            _ => {}
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
    This function calculates the display area for the overlay window to be rendered on.

    It will return the position and size of the overlay window.
    */
    pub fn calculate_display_area(
        event_loop: &ActiveEventLoop
    ) -> (PhysicalPosition<i32>, PhysicalSize<u32>) {
        // `min_x` and `max_y` track the position of where the overlay should be rendered.
        // This is typically the top-left coordinate across all monitors.
        // `max_x` and `max_y` track the largest `x + width` and `y + height` values,
        // which determines where the bottom-right corner across all monitors is.

        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for monitor in event_loop.available_monitors() {
            let size = monitor.size();
            let position = monitor.position();

            min_x = min_x.min(position.x);
            min_y = min_y.min(position.y);
            max_x = max_x.max(position.x + (size.width as i32));
            max_y = max_y.max(position.y + (size.height as i32));
        }

        // The difference between the top-left and bottom-right corner yields the resulting rect to be display.

        (
            PhysicalPosition::new(min_x, min_y),
            PhysicalSize::new((max_x - min_x) as u32, (max_y - min_y) as u32)
        )
    }
}


/**
Defines the input model for a Vertex in the shader.
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
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


// These indices will always make a quad where the initial vertex is the top-right point. 
const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];


struct State<'a> {
    size: PhysicalSize<u32>,

    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    clip: Option<data::Rect>,

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
            clip: None,

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

    fn create_surface_config(
        size: PhysicalSize<u32>,
        capabilities: wgpu::SurfaceCapabilities
    ) -> wgpu::SurfaceConfiguration {
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

    fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
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
                                // Default clear color is Black with 40% opacity.
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.4,
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
            
            // If there is a "clip" rect, this will call the shader code and provide the same clipping
            // area to "cut" a quad in the overlay.
            // Otherwise, the overlay will be fully rendered with the provided color attachment.
            if let Some(clip) = self.clip {
                // The initial point (0, 0) of the render area in WebGPU is the middle of the screen area.
                // By halving the width and height of the overlay, the initial point is acquired.
                let center_x = self.size.width as f32 / 2.0;
                let center_y = self.size.height as f32 / 2.0;

                // A transformation is applied to the clipping rect to remap it to the overlay's
                // coordinates. The top (1) and bottom (3) are multiplied by -1 to invert the coordinates. 
                let rect = clip.raw();
                let (left, top, right, bottom) = (
                    rect.0 as f32 - center_x,
                    (rect.1 as f32 - center_y) * -1.0,
                    rect.2 as f32 - center_x,
                    (rect.3 as f32 - center_y) * -1.0,
                );
                
                let vertices = &[
                    Vertex { position: [right / center_x, top / center_y] },
                    Vertex { position: [left / center_x, top / center_y] },
                    Vertex { position: [left / center_x, bottom / center_y] },
                    Vertex { position: [right / center_x, bottom / center_y] },
                ];

                let vertex_buffer = Self::create_vertex_buffer(&self.device, vertices);
                let index_buffer = Self::create_index_buffer(&self.device, INDICES);

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..INDICES.len() as _, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
