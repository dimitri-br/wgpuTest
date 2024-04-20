use crate::{Handle, MutHandle};
use super::commands::{Command, DrawCommand};
use crate::pipeline::Pipeline;

use std::fs::File;
use std::io::Read;
use crate::render_graph::{ResourceHandle, ResourceManager, ResourceType};
use crate::types::{Instance, Uniform, UniformBuffer, UniformBufferType, UniformSet, Vertex};

pub struct RenderNode {
    pub name: String,

    commands: Vec<Command>,

    compiled_commands: Vec<DrawCommand>,

    pipeline: Option<Pipeline>, // The pipeline that this node will use to render.

    static_uniform_set: Option<UniformSet>,
    dynamic_uniform_set: Option<UniformSet>,

    // Configs
    use_depth: bool,

    _device: Handle<wgpu::Device>,
    _queue: Handle<wgpu::Queue>,
}

impl RenderNode {
    pub fn new(name: String, _device: Handle<wgpu::Device>, _queue: Handle<wgpu::Queue>) -> Self {
        Self {
            name,
            commands: Vec::new(),
            compiled_commands: Vec::new(),
            pipeline: None,
            static_uniform_set: None,
            dynamic_uniform_set: None,

            use_depth: false,

            _device,
            _queue,
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn use_depth(&mut self, use_depth: bool) {
        self.use_depth = use_depth;
    }

    pub fn add_uniform_buffer<T: Uniform>(&mut self, data: &T, buffer: UniformBufferType) -> Option<Handle<UniformBuffer>> {
        let mut dynamic_uniform_buffer = None;

        match buffer {
            UniformBufferType::STATIC => {
                if let Some(static_uniform_set) = &mut self.static_uniform_set {
                    let uniform_buffer = UniformBuffer::new(self._device.clone(), self._queue.clone(), data);
                    static_uniform_set.add_uniform_buffer(&self._device, uniform_buffer);
                } else {
                    let uniform_buffer = UniformBuffer::new(self._device.clone(), self._queue.clone(), data);
                    let uniform_buffer = Handle::new(uniform_buffer);
                    let uniform_set = UniformSet::new(&self._device, vec![uniform_buffer]);
                    self.static_uniform_set = Some(uniform_set);

                    dynamic_uniform_buffer = Some(self.static_uniform_set.as_ref().unwrap().uniform_buffers[0].clone());

                }
            }
            UniformBufferType::DYNAMIC => {
                if let Some(dynamic_uniform_set) = &mut self.dynamic_uniform_set {
                    let uniform_buffer = UniformBuffer::new(self._device.clone(), self._queue.clone(), data);
                    dynamic_uniform_set.add_uniform_buffer(&self._device, uniform_buffer);
                    dynamic_uniform_buffer = Some(dynamic_uniform_set.uniform_buffers[dynamic_uniform_set.uniform_buffers.len() - 1].clone());
                } else {
                    let uniform_buffer = UniformBuffer::new(self._device.clone(), self._queue.clone(), data);
                    let uniform_buffer = Handle::new(uniform_buffer);
                    let uniform_set = UniformSet::new(&self._device, vec![uniform_buffer]);
                    self.dynamic_uniform_set = Some(uniform_set);
                    dynamic_uniform_buffer = Some(self.dynamic_uniform_set.as_ref().unwrap().uniform_buffers[0].clone());
                }
            }
        }

        dynamic_uniform_buffer
    }

    // Add an existing uniform buffer to the node
    pub fn add_uniform_buffer_handle(&mut self, buffer: Handle<UniformBuffer>, buffer_type: UniformBufferType) {
        match buffer_type {
            UniformBufferType::STATIC => {
                if let Some(static_uniform_set) = &mut self.static_uniform_set {
                    static_uniform_set.add_existing_uniform_buffer(buffer.clone());
                } else {
                    let uniform_set = UniformSet::new(&self._device, vec![buffer.clone()]);
                    self.static_uniform_set = Some(uniform_set);
                }
            }
            UniformBufferType::DYNAMIC => {
                if let Some(dynamic_uniform_set) = &mut self.dynamic_uniform_set {
                    dynamic_uniform_set.add_existing_uniform_buffer(buffer.clone());
                } else {
                    let uniform_set = UniformSet::new(&self._device, vec![buffer.clone()]);
                    self.dynamic_uniform_set = Some(uniform_set);
                }
            }
        }
    }

    pub(super) fn build_pipeline(&mut self, resource_manager: MutHandle<ResourceManager>) {
        let mut resource_manager = resource_manager.lock().unwrap();

        let mut shader_module = None;
        let mut bind_group_layouts = Vec::new();
        let mut vertex_buffer_layouts = vec![Vertex::desc()];
        let mut compiled_commands = Vec::new();

        // Get our bind group layouts from our uniform sets
        if let Some(static_uniform_set) = &self.static_uniform_set {
            bind_group_layouts.push(&static_uniform_set.bind_group_layout);
        }

        if let Some(dynamic_uniform_set) = &self.dynamic_uniform_set {
            println!("Adding dynamic uniform set");
            bind_group_layouts.push(&dynamic_uniform_set.bind_group_layout);
        }

        // Load all textures and meshes
        for command in self.commands.iter(){
            match command{
                Command::LoadShader(shader) => {
                    // Load the shader
                    let mut file = File::open(shader).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();

                    let module = self._device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some(shader),
                        source: wgpu::ShaderSource::Wgsl(contents.into()),
                    });

                    shader_module = Some(module);
                }
                Command::BindTexture(idx, texture_id) => {
                    let texture_handle = ResourceHandle::new(texture_id.clone(), ResourceType::Texture);
                    resource_manager.load_texture(texture_handle.clone(), texture_id.clone());
                    compiled_commands.push(DrawCommand::BindTexture(*idx, texture_handle));
                }
                Command::DrawMesh(mesh_id) => {
                    // Load the mesh
                    let mesh_handle = ResourceHandle::new(mesh_id.clone(), ResourceType::Mesh);
                    resource_manager.load_mesh(mesh_handle.clone(), mesh_id);

                    compiled_commands.push(DrawCommand::DrawMesh(mesh_handle));
                }
                Command::DrawMeshInstanced(mesh_id, transform_instances) => {
                    // add vertex_buffer_layouts.push(Instance::desc()); if it doesn't already exist
                    if !vertex_buffer_layouts.contains(&Instance::desc()) {
                        vertex_buffer_layouts.push(Instance::desc());
                    }

                    let mesh_handle = ResourceHandle::new(mesh_id.clone(), ResourceType::Mesh);

                    // Convert the transform instances to instances
                    let instances: Vec<Instance> = transform_instances.iter().map(|transform| transform.to_instance()).collect();
                    let instance_buffer = resource_manager.build_instance_buffer(&instances);

                    // Load the mesh
                    resource_manager.load_mesh(mesh_handle.clone(), mesh_id);

                    let mesh = resource_manager.get_mesh_mut(mesh_handle.clone()).unwrap_or_else(
                        || panic!("Mesh with id {} not found", mesh_id)
                    );

                    mesh.set_instances(&self._device.clone(), instance_buffer);

                    compiled_commands.push(DrawCommand::DrawMeshInstanced(mesh_handle));
                }
                _ => {}
            }
        }


        for command in compiled_commands.iter(){
            match command{
                DrawCommand::BindTexture(_, texture_handle) => {
                    let texture = resource_manager.get_texture(texture_handle.clone());
                    if let Some(texture) = texture {
                        bind_group_layouts.push(texture.get_bind_group_layout());
                    }
                },
                _ => {}
            }
        }

        let pipeline = Pipeline::new(self._device.clone(), shader_module.unwrap(),
                                     bind_group_layouts, vertex_buffer_layouts, self.use_depth);

        self.pipeline = Some(pipeline);
        self.compiled_commands = compiled_commands;
    }

    pub(super) fn execute(&self, id: usize, texture_view: &wgpu::TextureView,
                          resource_manager: MutHandle<ResourceManager>, encoder: &mut wgpu::CommandEncoder) {
        if let Some(pipeline) = &self.pipeline {

            let mut resource_manager = resource_manager.lock().unwrap();

            let depth_texture = resource_manager.load_depth_texture();

            let depth_texture = depth_texture.lock().unwrap();

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&self.name),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Only clear if we're the first node in the render graph
                        load: if id == 0 {
                            wgpu::LoadOp::Clear(wgpu::Color::BLACK)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: if self.use_depth {
                    Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: if id == 0 {
                                wgpu::LoadOp::Clear(1.0)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    })
                } else {
                    None
                },
                ..Default::default()
            });

            pipeline.bind_pipeline(&mut render_pass);

            // If either the static or dynamic uniform set is not None, bind them

            // 0 is reserved for the projection matrix + view + model matrix
            let using_static_uniform_set = self.static_uniform_set.is_some();

            if let Some(static_uniform_set) = &self.static_uniform_set {
                static_uniform_set.bind(0, &mut render_pass);
            }

            if let Some(dynamic_uniform_set) = &self.dynamic_uniform_set {
                dynamic_uniform_set.bind(
                    if using_static_uniform_set { 1 } else { 0 }, 
                    &mut render_pass);
            }


            for command in self.compiled_commands.iter() {
                match command {
                    DrawCommand::DrawMesh(mesh_id) => {
                        let mesh = resource_manager.get_mesh(mesh_id.clone());

                        if let Some(mesh) = mesh {
                            mesh.render(&mut render_pass);
                        }
                    }
                    DrawCommand::DrawMeshInstanced(mesh_id) => {
                        let mesh = resource_manager.get_mesh(mesh_id.clone());

                        if let Some(mesh) = mesh {
                            mesh.render_instanced(&mut render_pass);
                        }
                    }
                    DrawCommand::BindTexture(index, texture_id) => {
                        let texture = resource_manager.get_texture(texture_id.clone());

                        if let Some(texture) = texture {
                            texture.bind(*index, &mut render_pass);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
