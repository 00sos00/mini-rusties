use crate::{
    camera::Camera,
    cube::{Cube, CUBE_VERTICES},
    texture::Texture,
    vertex::Vertex,
};
use wgpu::{include_wgsl, util::DeviceExt};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

fn begin_render_pass(state: &State, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 4.0 / 255.0,
                    g: 4.0 / 255.0,
                    b: 4.0 / 255.0,
                    a: 1.0,
                }),
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&state.render_pipeline);
    render_pass.set_bind_group(0, &state.tree_texture_bind_group, &[]);
    render_pass.set_bind_group(1, &state.camera.bind_group, &[]);
    render_pass.set_bind_group(2, &state.cube.bind_group, &[]);
    render_pass.set_vertex_buffer(0, state.cube_vertex_buffer.slice(..));
    render_pass.draw(0..CUBE_VERTICES.len() as u32, 0..1);
}

pub struct State {
    pub input: WinitInputHelper,
    queue: wgpu::Queue,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    pub mouse_offset: (f32, f32),
    pub capture_mouse: bool,
    pub cube: Cube,
    tree_texture: Texture,
    tree_texture_bind_group: wgpu::BindGroup,
    pub cube_vertex_buffer: wgpu::Buffer,
    pub dt: f32,
    pub current_time: std::time::Instant,
    pub start_time: std::time::Instant,
    pub time_since_start: f32,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let input = WinitInputHelper::new();

        let instance = wgpu::Instance::new(wgpu::Backends::DX12);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        println!("{:#?}", adapter.get_info());

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: 500,
            height: 500,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let camera = Camera::new(&device, "Main Camera");
        let mut cube = Cube::new(&device, "Cube");

        cube.transform.scale(50.0, 50.0, 50.0);
        cube.transform.translate(0.0, 0.0, 100.0);

        let tree_texture_bytes = include_bytes!("tree.png");
        let tree_texture =
            Texture::from_bytes(&device, &queue, tree_texture_bytes, Some("Tree texture")).unwrap();

        let tree_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let tree_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &tree_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&tree_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&tree_texture.sampler),
                },
            ],
            label: Some("tree_texture_bind_group"),
        });

        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &tree_texture_bind_group_layout,
                    &camera.bind_group_layout,
                    &cube.bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
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
        });

        let cube_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            input,
            queue,
            device,
            config,
            surface,
            render_pipeline,
            camera,
            mouse_offset: (0.0, 0.0),
            capture_mouse: true,
            cube,
            tree_texture,
            tree_texture_bind_group,
            cube_vertex_buffer,
            dt: 0.0,
            current_time: std::time::Instant::now(),
            start_time: std::time::Instant::now(),
            time_since_start: 0.0,
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update_camera(&mut self) {
        if self.capture_mouse {
            self.camera.process_input(
                self.mouse_offset,
                &self.input,
                self.time_since_start,
                self.dt,
            );
            self.mouse_offset = (0.0, 0.0);
        }

        self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        self.camera.update_uniform_buffer(&self.queue);
    }

    fn update_cube(&mut self) {
        self.cube.transform.look_at(self.camera.transform.translation);
        self.cube.update_uniform_buffer(&self.queue);
    }

    pub fn update(&mut self) {
        self.update_cube();
        self.update_camera();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.time_since_start = self.start_time.elapsed().as_secs_f32();

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        begin_render_pass(self, &mut encoder, &view);

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}
