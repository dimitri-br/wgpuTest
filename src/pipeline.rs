use std::num::NonZeroU32;
use crate::Handle;

pub struct PipelineSettings{
    primitive_mode: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    multiview: Option<NonZeroU32>,
}

impl Default for PipelineSettings{
    fn default() -> Self {
        Self{
            primitive_mode: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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

            multiview: None
        }
    }
}

impl PipelineSettings{
    pub fn enable_depth_stencil(mut self) -> Self {
        self.depth_stencil = Some(
            wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }
        );

        self
    }
}

pub struct Pipeline{
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline{
    pub fn new(device: Handle<wgpu::Device>, shader: wgpu::ShaderModule,
               bind_group_layouts: Vec<&wgpu::BindGroupLayout>,
               vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout>, use_depth: bool) -> Self{
        let layout = Self::create_layout(device.clone(), bind_group_layouts);

        let pipeline = Self::create_pipeline(device.clone(), layout, shader,
                                             vertex_buffer_layouts, use_depth);

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
                       shader: wgpu::ShaderModule,
                       vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout>,
                       use_depth: bool) -> wgpu::RenderPipeline{

        let mut pipeline_settings = PipelineSettings::default();

        if use_depth{
            pipeline_settings = pipeline_settings.enable_depth_stencil();
        }

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: "vert_main",
                buffers: &vertex_buffer_layouts,
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
            primitive: pipeline_settings.primitive_mode,
            depth_stencil: pipeline_settings.depth_stencil,
            multisample: pipeline_settings.multisample,
            multiview: pipeline_settings.multiview,
        })
    }
}

