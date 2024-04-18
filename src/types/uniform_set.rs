use std::sync::Arc;
use wgpu::{BindGroup, BindGroupLayout};
use crate::{Handle};
use crate::types::uniform_buffer::UniformBuffer;

pub enum UniformBufferType{
    STATIC, // Uniform buffer that is only updated once, upon creation
    DYNAMIC, // Uniform buffer that is updated every frame
}

pub struct UniformSet{
    pub uniform_buffers: Vec<Handle<UniformBuffer>>,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl UniformSet{
    pub fn new(device: &wgpu::Device, uniform_buffers: Vec<Handle<UniformBuffer>>) -> Self{

        let (bind_group_layout, bind_group) = Self::create_bind_groups(device, &uniform_buffers);

        Self{
            uniform_buffers,
            bind_group_layout,
            bind_group,
        }
    }

    fn create_bind_groups<'a>(device: &'a wgpu::Device, uniform_buffers: &'a [Handle<UniformBuffer>]) -> (BindGroupLayout, BindGroup) {
        let mut bind_group_layout_entries = Vec::new();
        let mut bind_group_entries = Vec::new();

        for (i, uniform_buffer) in uniform_buffers.iter().enumerate(){
            bind_group_layout_entries.push(wgpu::BindGroupLayoutEntry{
                binding: i as u32,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            });

            bind_group_entries.push(wgpu::BindGroupEntry{
                binding: i as u32,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding{
                    buffer: &uniform_buffer.buffer,
                    offset: 0,
                    size: None
                })
            });
        }

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor{
                label: Some("uniform_set_bind_group_layout"),
                entries: &bind_group_layout_entries
            }
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor{
                layout: &bind_group_layout,
                entries: &bind_group_entries,
                label: Some("uniform_set_bind_group")
            }
        );

        (bind_group_layout, bind_group)
    }

    pub fn add_uniform_buffer(&mut self, device: &wgpu::Device, uniform_buffer: UniformBuffer){
        self.uniform_buffers.push(Arc::new(uniform_buffer));

        let (bind_group_layout, bind_group) = Self::create_bind_groups(device, &self.uniform_buffers);

        self.bind_group_layout = bind_group_layout;
        self.bind_group = bind_group;
    }

    pub fn bind<'a>(&'a self, index: u32, render_pass: &mut wgpu::RenderPass<'a>){
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }
}