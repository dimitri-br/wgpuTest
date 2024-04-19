use encase::{internal::WriteInto, ShaderType};
use wgpu::util::DeviceExt;
use crate::Handle;

pub trait Uniform{
    fn to_wgpu(&self) -> Vec<u8>;
}

pub struct UniformBuffer{
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) size: usize, // Size of the buffer in bytes (used for updating the buffer

    _device: Handle<wgpu::Device>,
    _queue: Handle<wgpu::Queue>,
}

impl UniformBuffer{
    pub(crate) fn new<T: Uniform>(device: Handle<wgpu::Device>, queue: Handle<wgpu::Queue>, data: &T) -> Self{
        let data = data.to_wgpu();

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some("Uniform Buffer"),
                contents: &data,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let size = data.len();

        Self{
            buffer,
            size,

            _device: device,
            _queue: queue,
        }
    }

    pub fn update<T: Uniform>(&self, data: &T){
        let data = data.to_wgpu();

        if data.len() != self.size{
            panic!("Uniform buffer size mismatch");
        }

        self._queue.write_buffer(&self.buffer, 0, &data);
    }
}