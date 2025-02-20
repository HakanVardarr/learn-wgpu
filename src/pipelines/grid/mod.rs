use vertex::Vertex;
use wgpu::{util::DeviceExt, PipelineCompilationOptions};

mod vertex;
const GRID_SIZE: f32 = 512.0;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex::new([0.8, 0.8]),
    Vertex::new([-0.8, 0.8]),
    Vertex::new([-0.8, -0.8]),
    Vertex::new([0.8, -0.8]),
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];

pub struct Grid {
    pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline,
    bind_groups: [wgpu::BindGroup; 2],
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    grid_state_input_buffer: wgpu::Buffer,
    grid_state_output_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    time: f32,
    step: u32,
}

impl Drop for Grid {
    fn drop(&mut self) {
        self.vertex_buffer.destroy();
        self.index_buffer.destroy();
        self.uniform_buffer.destroy();
        self.grid_state_input_buffer.destroy();
        self.grid_state_output_buffer.destroy();
    }
}

impl Grid {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../../../shader/grid.wgsl"));
        let compute_shader =
            device.create_shader_module(wgpu::include_wgsl!("../../../shader/grid_compute.wgsl"));

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("grid_vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("grid_vertex_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let grid = [GRID_SIZE, GRID_SIZE];
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("grid_uniform_buffer"),
            contents: bytemuck::cast_slice(&[grid]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let mut grid_state_input = [[0u32; GRID_SIZE as usize]; GRID_SIZE as usize];
        for y in 0..GRID_SIZE as usize {
            for x in 0..GRID_SIZE as usize {
                grid_state_input[y][x] = (fastrand::f32() > 0.6) as u32
            }
        }

        let grid_state_input_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("grid_state_input_buffer"),
                contents: bytemuck::cast_slice(&[grid_state_input]),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        let grid_state_output = [[0u32; GRID_SIZE as usize]; GRID_SIZE as usize];
        let grid_state_output_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("grid_state_output_buffer"),
                contents: bytemuck::cast_slice(&[grid_state_output]),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("grid_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_groups = [
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("grid_bind_group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: grid_state_input_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: grid_state_output_buffer.as_entire_binding(),
                    },
                ],
            }),
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("grid_bind_group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: grid_state_output_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: grid_state_input_buffer.as_entire_binding(),
                    },
                ],
            }),
        ];

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("grid_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("grid_compute_pipeline"),
            layout: Some(&pipeline_layout),
            module: &compute_shader,
            entry_point: Some("compute_main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("grid_render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            compute_pipeline,
            bind_groups,
            grid_state_input_buffer,
            grid_state_output_buffer,
            vertex_buffer,
            uniform_buffer,
            index_buffer,
            index_count: INDICES.len() as u32,
            time: 0.0,
            step: 0,
        }
    }

    pub fn update(&mut self, dt: f32, compute_pass: &mut wgpu::ComputePass) {
        self.time += dt;
        if self.time > 0.01667 {
            self.time = 0.0;

            let workgroup_count = GRID_SIZE / 8.0;
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_groups[(self.step % 2) as usize], &[]);
            compute_pass.dispatch_workgroups(workgroup_count as u32, workgroup_count as u32, 1);

            self.step += 1;
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_groups[(self.step % 2) as usize], &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_count, 0, 0..(GRID_SIZE * GRID_SIZE) as u32);
    }
}
