use wgpu::util::DeviceExt;

use super::Instance;

/// A wrapper around a wgpu::Buffer that holds a list of instances and a bind group.
/// This is used to render multiple instances of the same mesh.
/// This provides an easy way to render multiple instances of the same mesh,
/// as well as to manage and update the instances.
pub struct InstanceBuffer{
    pub instances: Vec<Instance>,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl InstanceBuffer{
    /// Create a new instance buffer from a list of instances.
    pub fn new(device: &wgpu::Device, instances: Vec<Instance>) -> Self{
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor{
                label: Some("instance_buffer_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry{
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ]
            }
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor{
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry{
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding{
                            buffer: &buffer,
                            offset: 0,
                            size: None,
                        })
                    }
                ],
                label: Some("instance_buffer_bind_group"),
            }
        );

        Self{
            instances,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn get_instance_count(&self) -> usize{
        self.instances.len()
    }

    /// Update the instance buffer with a new list of instances.
    pub fn update(&self, queue: &wgpu::Queue){
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.instances));
    }

    pub fn update_slice(&self, queue: &wgpu::Queue, start: usize, end: usize) {
        queue.write_buffer(&self.buffer, (start * std::mem::size_of::<Instance>()) as wgpu::BufferAddress, bytemuck::cast_slice(&self.instances[start..end]));
    }

    pub fn update_instance(&mut self, index: usize, instance: Instance){
        self.instances[index] = instance;
    }

    pub fn bind_as_group<'a>(&'a self, index: u32, render_pass: &mut wgpu::RenderPass<'a>){
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }

    pub fn bind_as_buffer<'a>(&'a self, index: u32, render_pass: &mut wgpu::RenderPass<'a>){
        render_pass.set_vertex_buffer(1, self.buffer.slice(..));
    }

}