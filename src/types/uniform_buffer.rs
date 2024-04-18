use wgpu::util::DeviceExt;
use crate::Handle;

pub struct UniformBuffer{
    pub(crate) buffer: wgpu::Buffer,

    _device: Handle<wgpu::Device>,
    _queue: Handle<wgpu::Queue>,
}

impl UniformBuffer{
    pub(crate) fn new<T: bytemuck::Pod>(device: Handle<wgpu::Device>, queue: Handle<wgpu::Queue>, data: T) -> Self{
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self{
            buffer,

            _device: device,
            _queue: queue,
        }
    }

    pub fn update<T: bytemuck::Pod>(&self, data: T){
        self._queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[data]));
    }
}