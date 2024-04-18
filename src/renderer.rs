use crate::surface_wrapper::SurfaceWrapper;
use crate::Handle;
use crate::render_graph::{RenderGraph, RenderNode};
use crate::device_handler::DeviceHandler;
use crate::instance_handler::InstanceHandler;
use anyhow::Result;
use log::info;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use crate::render_graph::ResourceManager;

pub struct Renderer {
    window: Handle<winit::window::Window>,

    _instance_handler: InstanceHandler,
    device_handler: DeviceHandler,

    surface_wrapper: SurfaceWrapper,
    size: PhysicalSize<u32>,

    render_graph: RenderGraph,
    resource_manager: ResourceManager,
}

impl Renderer {
    fn init_logger() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            // We keep wgpu at Error level, as it's very noisy.
            .filter_module("wgpu_core", log::LevelFilter::Info)
            .filter_module("wgpu_hal", log::LevelFilter::Error)
            .filter_module("naga", log::LevelFilter::Error)
            .parse_default_env()
            .init();
    }

    pub fn new(_window: winit::window::Window) -> Result<Self> {
        Self::init_logger();

        info!("Initializing renderer");

        let window = Arc::new(_window); // Store the window in an Arc to ensure it is not dropped
        let size = window.inner_size();

        let instance_handler = InstanceHandler::new();
        let instance = instance_handler.get_instance();

        let mut surface_wrapper = SurfaceWrapper::new(instance.clone(), window.clone());

        let device_handler = DeviceHandler::new(instance, surface_wrapper.get_surface());

        surface_wrapper.configure(
            device_handler.get_adapter(),
            device_handler.get_device(),
            window.inner_size(),
        );

        let render_graph = RenderGraph::new();

        let resource_manager = ResourceManager::new(
            device_handler.get_device(),
            device_handler.get_queue());

        info!("Successfully initialized renderer");

        Ok(Self {
            window,
            _instance_handler: instance_handler,
            device_handler,
            surface_wrapper,
            size,
            render_graph,
            resource_manager
        })
    }

    pub fn initialize(&mut self) {
        self.render_graph.build(&mut self.resource_manager);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.surface_wrapper
            .resize(self.device_handler.get_device(), new_size);
    }

    pub fn update(&mut self, event: Event<()>) {
        // Handle rendering events here
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        self.resize(new_size);
                        self.window.request_redraw();
                    }
                    // On RedrawRequested, request a redraw
                    WindowEvent::RedrawRequested => {
                        self.render();

                        self.window.request_redraw();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn render(&self) {
        // Render the scene here
        // Get the next frame from the surface
        let frame = self
            .surface_wrapper
            .acquire_frame(self.device_handler.get_device());

        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder
        let mut encoder = self
            .device_handler
            .begin_command_buffer(Some("Command Encoder"));

        // Iterate over the render graph and execute each node
        self.render_graph.execute(&frame_view, &self.resource_manager, &mut encoder);

        // Submit the render pass
        self.device_handler.submit_command_encoder(encoder);

        // Present the frame
        frame.present();
    }

    pub fn get_render_node(&mut self, name: String) -> RenderNode{
        RenderNode::new(name, self.device_handler.get_device(), self.device_handler.get_queue())
    }

    pub fn add_render_node(&mut self, node: RenderNode) {
        self.render_graph.add_node(node);
    }
}
