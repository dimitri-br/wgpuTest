use crate::Handle;
use std::sync::Arc;
use wgpu::Instance;

pub struct InstanceHandler {
    // wgpu
    instance: Handle<Instance>,
}

impl InstanceHandler {
    pub fn new() -> Self {
        //let backends = wgpu::util::backend_bits_from_env().unwrap_or_default();
        let backends = wgpu::Backends::VULKAN;
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version = wgpu::util::gles_minor_version_from_env().unwrap_or_default();
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends,
            flags: wgpu::InstanceFlags::from_build_config().with_env(),
            dx12_shader_compiler,
            gles_minor_version,
        });

        Self {
            instance: Arc::new(instance),
        }
    }

    pub fn get_instance(&self) -> Handle<Instance> {
        self.instance.clone()
    }
}
