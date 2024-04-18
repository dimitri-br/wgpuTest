use crate::Handle;

pub struct Pipeline{
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline{
    pub fn new(device: Handle<wgpu::Device>, shader: wgpu::ShaderModule, bind_group_layouts: Vec<&wgpu::BindGroupLayout>) -> Self{
        let layout = Self::create_layout(device.clone(), bind_group_layouts);

        let pipeline = Self::create_pipeline(device.clone(), layout, shader);

        Self{
            pipeline
        }
    }

    pub fn bind_pipeline<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>){
        render_pass.set_pipeline(&self.pipeline);
    }

    fn create_layout(device: Handle<wgpu::Device>, bind_group_layouts: Vec<&wgpu::BindGroupLayout>) -> wgpu::PipelineLayout{
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Pipeline Layout"),
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
        })
    }

    fn create_pipeline(device: Handle<wgpu::Device>, layout: wgpu::PipelineLayout,
                       shader: wgpu::ShaderModule) -> wgpu::RenderPipeline{
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: "vert_main",
                buffers: &[
                    crate::types::Vertex::desc()
                ],
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState{
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState{
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}

