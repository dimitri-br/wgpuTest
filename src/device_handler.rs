use crate::Handle;
use log::error;
use std::sync::Arc;
use wgpu::{Adapter, CommandEncoder, Device, Instance, Queue, Surface};

pub struct DeviceHandler {
    adapter: Handle<Adapter>,
    device: Handle<Device>,
    queue: Handle<Queue>,
}

impl DeviceHandler {
    pub fn new(instance: Handle<Instance>, surface: Handle<Surface>) -> Self {
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap_or_else(|| {
            error!("Failed to request adapter");
            panic!("Failed to request adapter");
        });

        let (device, queue) = pollster::block_on(async {
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::empty(),
                        required_limits: if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        },
                        label: Some("Device"),
                    },
                    None,
                )
                .await
                .unwrap_or_else(|e| {
                    error!("Failed to request device: {:?}", e);
                    panic!("Failed to request device: {:?}", e);
                })
        });

        Self {
            adapter: Arc::new(adapter),
            device: Arc::new(device),
            queue: Arc::new(queue),
        }
    }

    pub fn get_device(&self) -> Handle<Device> {
        self.device.clone()
    }

    pub fn get_adapter(&self) -> Handle<Adapter> {
        self.adapter.clone()
    }

    pub fn get_queue(&self) -> Handle<Queue> {
        self.queue.clone()
    }

    pub fn begin_command_buffer(&self, label: Option<&'static str>) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: label.or(Some("Command Encoder")),
            })
    }

    pub fn submit_command_encoder(&self, encoder: CommandEncoder) -> wgpu::SubmissionIndex {
        self.queue.submit(std::iter::once(encoder.finish()))
    }
}
