use std::sync::Arc;
use std::sync::LazyLock;
use crate::core::render::font::FONT_DATA;
use crate::core::{color::Color, common::intersect_rects, event::DrawCommand, kernel::GuiWindow, render::rendercommands::Renderer, shapes::Rect, size::Size};
use wgpu::util::DeviceExt;
use wgpu_text::glyph_brush::{Section, Text};
use wgpu_text::BrushBuilder;

static SRGB_TO_LINEAR_LUT: LazyLock<[f32; 256]> = LazyLock::new(|| {
    let mut lut = [0.0; 256];
    for i in 0..256 {
        let c = i as f32 / 255.0;
        lut[i] = if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        };
    }
    lut
});

#[inline(always)]
pub fn srgb_u8_to_linear(val: u8) -> f32 {
    SRGB_TO_LINEAR_LUT[val as usize]
}

impl From<Color> for wgpu::Color {
    fn from(c: Color) -> Self {
        wgpu::Color {
            r: srgb_u8_to_linear(c.r) as f64,
            g: srgb_u8_to_linear(c.g) as f64,
            b: srgb_u8_to_linear(c.b) as f64,
            a: c.a as f64 / 255.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

struct TextCommand {
    rect: Rect,
    clip: Rect,
    color: [f32; 4],
    font_size: f32,
    text: Arc<String>,
}

pub struct WGPURenderConfig {
    pub is_partial_render: bool,
    surface: Option<wgpu::Surface<'static>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    config: Option<wgpu::SurfaceConfiguration>,
    is_surface_configured: bool,

    current_frame: Option<wgpu::SurfaceTexture>,
    current_view: Option<wgpu::TextureView>,
    current_encoder: Option<wgpu::CommandEncoder>,
    bind_group: Option<wgpu::BindGroup>,
    screen_buffer: Option<wgpu::Buffer>,

    text_brush: Option<wgpu_text::TextBrush<wgpu_text::glyph_brush::ab_glyph::FontArc>>,
    text_commands: Vec<TextCommand>,

    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>, 
    
    vertex_buffer_cap: usize,
    index_buffer_cap: usize,

    pub num_vertices: u32,
    pub num_indices: u32,

    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Default for WGPURenderConfig {
    fn default() -> Self {
        Self {
            is_partial_render: false,
            surface: None,
            device: None,
            queue: None,
            config: None,
            is_surface_configured: false,

            current_frame: None,
            current_view: None,
            current_encoder: None,
            bind_group: None,
            screen_buffer: None,

            text_brush: None,
            text_commands: Vec::with_capacity(16),

            render_pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            vertex_buffer_cap: 0,
            index_buffer_cap: 0,
            num_vertices: 0,
            num_indices: 0,
            vertices: Vec::with_capacity(512),
            indices: Vec::with_capacity(256),
        }
    }
}

impl Renderer for WGPURenderConfig {
    fn is_partial_render(&self) -> bool {
        self.is_partial_render
    }

    fn init_window(&mut self, window: Arc<dyn GuiWindow>, size: Size) {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: true,
        })).expect("Failed to get wgpu adapter");

        let (device, queue) = pollster::block_on(
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("GUI Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                },
            )
        ).expect("Failed to get WGPU Device and Queue");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let screen_size = [size.width as f32, size.height as f32];

        let screen_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Screen Size Uniform Buffer"),
            contents: bytemuck::cast_slice(&screen_size),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Screen Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Screen Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::OVER,
                        alpha: wgpu::BlendComponent::OVER,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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
            multiview_mask: None,
            cache: None,
        });

        let font = wgpu_text::glyph_brush::ab_glyph::FontArc::try_from_slice(FONT_DATA)
            .expect("Failed to load font");

        let text_brush = BrushBuilder::using_font(font)
            .build(&device, config.width, config.height, config.format);

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: &[],
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: &[],
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);
        self.screen_buffer = Some(screen_buffer);
        self.text_brush = Some(text_brush);
        self.bind_group = Some(bind_group);
        self.render_pipeline = Some(render_pipeline);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.is_surface_configured = true;
    }

    fn resize(&mut self, size: Size) {
        let width = size.width as u32;
        let height = size.height as u32;

        if width > 0 && height > 0 {
            if let (Some(config), Some(surface), Some(device)) = 
                (&mut self.config, &self.surface, &self.device) 
            {
                if config.width != width || config.height != height {
                    config.width = width;
                    config.height = height;
                    surface.configure(device, config);
                    self.is_surface_configured = true;

                    if let (Some(queue), Some(screen_buffer)) = (&self.queue, &self.screen_buffer) {
                        let screen_size = [width as f32, height as f32];
                        queue.write_buffer(screen_buffer, 0, bytemuck::cast_slice(&screen_size));
                    }
                    if let (Some(brush), Some(queue)) = (&mut self.text_brush, &self.queue) {
                        brush.resize_view(width as f32, height as f32, queue);
                    }
                }
            }
        }
    }

    fn begin(&mut self) {
        if !self.is_surface_configured { return; }

        if let (Some(config), Some(surface), Some(device)) = 
            (&mut self.config, &mut self.surface, &mut self.device) 
        {
            let output = match surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(texture) => texture,
                wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,
                wgpu::CurrentSurfaceTexture::Outdated => {
                    eprintln!("Surface outdated, reconfiguring...");
                    surface.configure(device, config);
                    return;
                }
                _ => { eprintln!("Surface texture error"); return; },
            };

            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Frame Encoder"),
            });

            self.current_frame = Some(output);
            self.current_view = Some(view);
            self.current_encoder = Some(encoder);
        }
    }

    fn flush(&mut self) {
        if let (Some(frame), Some(encoder), Some(queue)) = (
            self.current_frame.take(),
            self.current_encoder.take(),
            &self.queue,
        ) {
            queue.submit(std::iter::once(encoder.finish()));
            queue.present(frame);

            self.current_view.take();

            let current_lenv = self.vertices.len();
            let current_leni = self.indices.len();
            let current_lent = self.text_commands.len();
            self.vertices.clear();
            self.indices.clear();
            self.text_commands.clear();

            if self.vertices.capacity() > 2048 && current_lenv < self.vertices.capacity() / 4 {
                self.vertices.shrink_to((current_lenv * 2).max(512));
            }
            if self.indices.capacity() > 1024 && current_leni < self.indices.capacity() / 4 {
                self.indices.shrink_to((current_leni * 2).max(256));
            }
            if self.text_commands.capacity() > 64 && current_lent < self.text_commands.capacity() / 4 {
                self.text_commands.shrink_to((current_lent * 2).max(16));
            }
        }
    }

    fn rendercl(&mut self, cl: &Vec<DrawCommand>) {
        if !self.is_surface_configured { 
            return; 
        }

        for da in cl {
            match da {
                DrawCommand::Rect(rect, color, clip) => {
                    self.drawrect(*rect, *color, *clip);
                }
                DrawCommand::Text(rect, color, textcolor, text, cliprect, font_size, padding) => {
                    self.drawtext(*rect, *color, *textcolor, text.clone(), *cliprect, *font_size, *padding);
                }
            }
        }

        if self.vertices.is_empty() && self.text_commands.is_empty() {
            return;
        }

        if let (Some(config), Some(queue), Some(screen_buffer)) = (&self.config, &self.queue, &self.screen_buffer) {
            let current_size = [config.width as f32, config.height as f32];
            queue.write_buffer(screen_buffer, 0, bytemuck::cast_slice(&current_size));
        }

        if let (Some(brush), Some(device), Some(queue)) = (&mut self.text_brush, &self.device, &self.queue) {
            if !self.text_commands.is_empty() {
                let sections: Vec<Section> = self.text_commands.iter().map(|cmd| {
                    let offset_x = 2.0; 

                    Section::default()
                        .add_text(
                            Text::new(&cmd.text)
                                .with_scale(cmd.font_size)
                                .with_color(cmd.color),
                        )
                        .with_screen_position((cmd.rect.x as f32 + offset_x, cmd.rect.y as f32))
                        .with_bounds((cmd.rect.width as f32, cmd.rect.height as f32))
                }).collect();

                brush.queue(device, queue, &sections).unwrap();
            }
        }

        if let (Some(device), Some(queue), Some(view), Some(encoder), Some(render_pipeline), Some(bind_group)) = (
            &self.device,
            &self.queue,
            &self.current_view,
            &mut self.current_encoder,
            &self.render_pipeline,
            &self.bind_group,
        ) {
            let v_data = bytemuck::cast_slice(&self.vertices);
            let i_data = bytemuck::cast_slice(&self.indices);

            if self.vertices.len() > self.vertex_buffer_cap {
                self.vertex_buffer_cap = (self.vertices.len() * 2).max(1024);
                self.vertex_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Dynamic Vertex Buffer"),
                    size: (self.vertex_buffer_cap * std::mem::size_of::<Vertex>()) as u64,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }));
            }

            if self.indices.len() > self.index_buffer_cap {
                self.index_buffer_cap = (self.indices.len() * 2).max(2048);
                self.index_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Dynamic Index Buffer"),
                    size: (self.index_buffer_cap * std::mem::size_of::<u32>()) as u64,
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }));
            }

            let v_buf = self.vertex_buffer.as_ref().unwrap();
            let i_buf = self.index_buffer.as_ref().unwrap();

            if !self.vertices.is_empty() {
                queue.write_buffer(v_buf, 0, v_data);
                queue.write_buffer(i_buf, 0, i_data);
            }

            let num_indices = self.indices.len() as u32;
            let target_width = self.config.as_ref().map(|c| c.width).unwrap_or(0);
            let target_height = self.config.as_ref().map(|c| c.height).unwrap_or(0);

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });

                if !self.vertices.is_empty() {
                    render_pass.set_pipeline(render_pipeline);
                    render_pass.set_bind_group(0, bind_group, &[]);
                    render_pass.set_vertex_buffer(0, v_buf.slice(..v_data.len() as u64));
                    render_pass.set_index_buffer(i_buf.slice(..i_data.len() as u64), wgpu::IndexFormat::Uint32); 
                    render_pass.draw_indexed(0..num_indices, 0, 0..1);
                }

                if let Some(brush) = &mut self.text_brush {
                    if !self.text_commands.is_empty() {
                        for cmd in &self.text_commands {
                            let clip = intersect_rects(
                                cmd.clip,
                                Rect { x: 0, y: 0, width: target_width as i32, height: target_height as i32 }
                            );

                            if let Some(c) = clip {
                                if c.width > 0 && c.height > 0 {
                                    render_pass.set_scissor_rect(
                                        c.x.max(0) as u32,
                                        c.y.max(0) as u32,
                                        c.width as u32,
                                        c.height as u32,
                                    );
                                    brush.draw(&mut render_pass);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn drawrect(&mut self, rect: Rect, color: Color, clip: Rect) {
        if color.a == 0 { return; }
        
        let r = match intersect_rects(rect, clip) {
            Some(r) => r,
            None => return,
        };

        let base_index = self.vertices.len() as u32;

        let c = [
            srgb_u8_to_linear(color.r),
            srgb_u8_to_linear(color.g),
            srgb_u8_to_linear(color.b),
        ];

        let x = r.x as f32;
        let y = r.y as f32;
        let w = r.width as f32;
        let h = r.height as f32;

        self.vertices.push(Vertex { position: [x, y, 0.0], color: c });
        self.vertices.push(Vertex { position: [x + w, y, 0.0], color: c });
        self.vertices.push(Vertex { position: [x, y + h, 0.0], color: c });
        self.vertices.push(Vertex { position: [x + w, y + h, 0.0], color: c });

        self.indices.extend_from_slice(&[
            base_index + 0, base_index + 1, base_index + 2,
            base_index + 1, base_index + 3, base_index + 2,
        ]);
    }

    fn drawtext(
        &mut self,
        rect: Rect,
        color: Color,
        textcolor: Color,
        text: Arc<String>,
        clip: Rect,
        font_size: i32,
        _padding: f32,
    ) {
        if color.a > 0 {
            self.drawrect(rect, color.lighter(15), clip);
        }

        if text.is_empty() {
            return;
        }

        let text_color = [
            srgb_u8_to_linear(textcolor.r),
            srgb_u8_to_linear(textcolor.g),
            srgb_u8_to_linear(textcolor.b),
            textcolor.a as f32 / 255.0,
        ];

        //Font size fix
        let scale_ratio = 1.333_333_4;
        let scaled_size = (font_size as f32 * scale_ratio).max(1.0);

        self.text_commands.push(TextCommand {
            rect,
            clip,
            color: text_color,
            font_size: scaled_size,
            text,
        });
    }
}