use crate::{Handle, MutHandle};

use log::{error, info};
use std::sync::{Arc, Mutex};
use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct SurfaceWrapper {
    surface: Handle<Surface<'static>>,
    surface_config: Option<MutHandle<SurfaceConfiguration>>,
}

impl SurfaceWrapper {
    pub fn new(instance: Handle<Instance>, window: Handle<Window>) -> Self {
        let surface = instance.create_surface(window).unwrap_or_else(|err| {
            error!("Failed to create surface: {:?}", err);
            panic!("Failed to create surface: {:?}", err)
        });

        Self {
            surface: Arc::new(surface),
            surface_config: None,
        }
    }

    pub fn get_surface(&self) -> Handle<Surface> {
        self.surface.clone()
    }

    pub fn get_configuration(&self) -> MutHandle<SurfaceConfiguration> {
        self.surface_config.clone().unwrap_or_else(|| {
            error!("Surface configuration is not set.");
            panic!("Surface configuration is not set.");
        })
    }

    pub fn configure(
        &mut self,
        adapter: Handle<Adapter>,
        device: Handle<Device>,
        size: PhysicalSize<u32>,
    ) {
        // Get the capabilities of the surface with the selected adapter.
        let surface_caps = self.surface.get_capabilities(&adapter);

        // Choose an sRGB format from the available formats, or fall back to the first format if none are sRGB.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or_else(|| {
                error!("No sRGB format found. Falling back to first format.");
                surface_caps.formats[0]
            });

        // Configure the surface with the desired settings.
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };

        // Apply the configuration to the surface.
        self.surface.configure(&device, &config);

        // Update the stored configuration
        self.surface_config = Some(Arc::new(Mutex::new(config)));
    }

    pub fn resize(&mut self, device: Handle<Device>, new_size: winit::dpi::PhysicalSize<u32>) {
        // Do checks here to ensure that the new size is valid
        if new_size.width == 0x0 || new_size.height == 0x0 {
            error!("Invalid window size: {:?}", new_size);
            return;
        }

        if let Some(config) = &mut self.surface_config {
            info!("Resizing window to {:?}", new_size);

            let config = &mut *config.lock().unwrap();

            config.width = new_size.width;
            config.height = new_size.height;
            self.surface.configure(&device, config);
        }
    }

    pub fn acquire_frame(&self, device: Handle<Device>) -> wgpu::SurfaceTexture {
        match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Timeout) => {
                // Attempt to get the frame again
                self.surface.get_current_texture().unwrap_or_else(|err| {
                    error!("Failed to acquire frame: {:?}", err);
                    panic!("Failed to acquire frame: {:?}", err)
                })
            }
            Err(
                wgpu::SurfaceError::Outdated
                | wgpu::SurfaceError::Lost
                | wgpu::SurfaceError::OutOfMemory,
            ) => {
                // Resize the surface and attempt to get the frame again
                self.surface.configure(
                    &device,
                    &self.surface_config
                        .clone()
                        .unwrap_or_else(|| {
                            error!("Surface configuration is not set.");
                            panic!("Surface configuration is not set.");
                        })
                        .lock()
                        .unwrap(),
                );

                self.surface.get_current_texture().unwrap_or_else(|err| {
                    error!("Failed to acquire frame: {:?}", err);
                    panic!("Failed to acquire frame: {:?}", err)
                })
            }
        }
    }
}
