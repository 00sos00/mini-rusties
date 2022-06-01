use crate::{
    camera::Camera,
    texture::Texture,
    vertex::Vertex,
    TRI_INDICES, TRI_VERTICES,
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
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 1.0,
                }),
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&state.render_pipeline);
    render_pass.set_bind_group(0, &state.tree_texture_bind_group, &[]);
    render_pass.set_bind_group(1, &state.camera_bind_group, &[]);
    render_pass.set_vertex_buffer(0, state.tri_vertex_buffer.slice(..));
    render_pass.set_index_buffer(state.tri_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..state.tri_num_indices, 0, 0..1);
}

pub struct State {
    pub input: WinitInputHelper,
    queue: wgpu::Queue,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
    render_pipeline: wgpu::RenderPipeline,
    pub camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    tree_texture: Texture,
    tree_texture_bind_group: wgpu::BindGroup,
    pub tri_num_indices: u32,
    pub tri_index_buffer: wgpu::Buffer,
    pub tri_vertex_buffer: wgpu::Buffer,
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

        let camera = Camera::new();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera.uniform()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let tree_texture_bytes = include_bytes!("happy-tree.png");
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
                bind_group_layouts: &[&tree_texture_bind_group_layout, &camera_bind_group_layout],
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

        let tri_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(TRI_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let tri_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(TRI_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let tri_num_indices = TRI_INDICES.len() as u32;

        Self {
            input,
            queue,
            device,
            config,
            surface,
            render_pipeline,
            camera,
            camera_buffer,
            camera_bind_group,
            tree_texture,
            tree_texture_bind_group,
            tri_num_indices,
            tri_index_buffer,
            tri_vertex_buffer,
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {
        self.camera.process_input(&self.input);
        self.camera.aspect = self.config.width as f32 / self.config.height as f32;

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform()]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
