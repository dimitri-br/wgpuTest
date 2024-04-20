use crate::{Handle, MutHandle};
use crate::pipeline::Pipeline;
use crate::render_graph::{DrawCommand, ResourceHandle, ResourceManager};

pub struct Material{
    pub name: String,
    base_material: Option<ResourceHandle>, // Base material to inherit from (unless this is a base material)
    pipeline: Option<Pipeline>, // Pipeline for this material - only if this is a base material

    // Material properties
    pub color: [f32; 4],
    pub roughness: f32,
    pub specular: [f32; 3],
    pub albedo_texture: Option<ResourceHandle>,
    pub normal_texture: Option<ResourceHandle>,
}

impl Material{
    pub fn new<T: Into<String>>(name: T,
                                base_material: Option<ResourceHandle>,
                                color: [f32; 4],
                                roughness: f32,
                                specular: [f32; 3],
                                albedo_texture: Option<ResourceHandle>,
                                normal_texture: Option<ResourceHandle>) -> Self{
        Self{
            name: name.into(),
            base_material,
            pipeline: None,

            color,
            roughness,
            specular,
            albedo_texture,
            normal_texture,
        }
    }

    pub fn new_base<T: Into<String>>(name: T,
                                     color: [f32; 4],
                                     roughness: f32,
                                     specular: [f32; 3],
                                     albedo_texture: Option<ResourceHandle>,
                                     normal_texture: Option<ResourceHandle>) -> Self{
        Self{
            name: name.into(),
            base_material: None,
            pipeline: None,

            color,
            roughness,
            specular,
            albedo_texture,
            normal_texture,
        }
    }

    pub fn build_material(&mut self, device: Handle<wgpu::Device>, shader: wgpu::ShaderModule,
                          bind_group_layouts: Vec<&wgpu::BindGroupLayout>,
                          vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout>, use_depth: bool){
        let pipeline = Pipeline::new(
            device,
            shader,
            bind_group_layouts,
            vertex_buffer_layouts,
            use_depth
        );

        self.pipeline = Some(pipeline);
    }

    pub fn bind_pipeline<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>){
        if let Some(pipeline) = &self.pipeline{
            pipeline.bind_pipeline(render_pass);
        }
    }

    pub fn is_base(&self) -> bool{
        self.base_material.is_none()
    }

    pub fn generate_draw_commands(&self) -> Vec<DrawCommand>{
        let mut commands = Vec::new();

        // Create draw commands for our textures
        if let Some(albedo) = &self.albedo_texture{
            commands.push(DrawCommand::BindTexture(0, albedo.clone()));
        }

        if let Some(normal) = &self.normal_texture{
            commands.push(DrawCommand::BindTexture(1, normal.clone()));
        }

        commands
    }
}

impl Eq for Material{}

impl PartialEq for Material{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}