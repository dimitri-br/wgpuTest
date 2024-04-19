use crate::Handle;
use super::commands::Command;
use crate::pipeline::Pipeline;

use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use wgpu::include_spirv;
use crate::render_graph::ResourceManager;
use crate::types::{Instance, UniformBuffer, UniformBufferType, UniformSet, Vertex};

pub struct RenderNode {
    pub name: String,

    commands: Vec<Command>,
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

    pub fn add_uniform_buffer<T: bytemuck::Pod>(&mut self, data: T, buffer: UniformBufferType) -> Option<Handle<UniformBuffer>> {
        let mut dynamic_uniform_buffer = None;

        match buffer {
            UniformBufferType::STATIC => {
                if let Some(static_uniform_set) = &mut self.static_uniform_set {
                    let uniform_buffer = UniformBuffer::new(self._device.clone(), self._queue.clone(), data);
                    static_uniform_set.add_uniform_buffer(&self._device, uniform_buffer);
                } else {
                    let uniform_buffer = UniformBuffer::new(self._device.clone(), self._queue.clone(), data);
                    let uniform_buffer = Arc::new(uniform_buffer);
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
                    let uniform_buffer = Arc::new(uniform_buffer);
                    let uniform_set = UniformSet::new(&self._device, vec![uniform_buffer]);
                    self.dynamic_uniform_set = Some(uniform_set);
                    dynamic_uniform_buffer = Some(self.dynamic_uniform_set.as_ref().unwrap().uniform_buffers[0].clone());
                }
            }
        }

        dynamic_uniform_buffer
    }

    pub(super) fn build_pipeline(&mut self, resource_manager: &mut ResourceManager) {
        let mut shader_module = None;
        let mut bind_group_layouts = Vec::new();
        let mut vertex_buffer_layouts = vec![Vertex::desc()];

        // Get our bind group layouts from our uniform sets
        if let Some(static_uniform_set) = &self.static_uniform_set {
            bind_group_layouts.push(&static_uniform_set.bind_group_layout);
        }

        if let Some(dynamic_uniform_set) = &self.dynamic_uniform_set {
            bind_group_layouts.push(&dynamic_uniform_set.bind_group_layout);
        }

        // Load all textures and meshes
        for command in self.commands.iter(){
            match command{
                Command::BindTexture(_, texture_id) => {
                    resource_manager.load_texture(texture_id.clone(), texture_id.clone());
                }
                Command::DrawMesh(mesh_id) => {
                    // Load the mesh
                    resource_manager.load_mesh(mesh_id.clone(), mesh_id);
                }
                Command::DrawMeshInstanced(mesh_id, _, transform_instances) => {
                    // add vertex_buffer_layouts.push(Instance::desc()); if it doesn't already exist
                    if !vertex_buffer_layouts.contains(&Instance::desc()) {
                        vertex_buffer_layouts.push(Instance::desc());
                    }

                    // Load the mesh
                    resource_manager.load_mesh(mesh_id.clone(), mesh_id);

                    // Convert the transform instances to instances
                    let instances: Vec<Instance> = transform_instances.iter().map(|transform| transform.to_instance()).collect();

                    let buffer = resource_manager.build_instance_buffer(&instances);

                    let mesh = resource_manager.get_mesh_mut(mesh_id.clone()).unwrap_or_else(
                        || panic!("Mesh with id {} not found", mesh_id)
                    );

                    mesh.set_instances(instances, buffer);
                }
                _ => {}
            }
        }

        // Now generate needed values to render
        for command in self.commands.iter() {
            match command {
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
                Command::BindTexture(_, texture_id) => {
                    // Load the texture
                    let texture = resource_manager.get_texture(texture_id.clone());
                    println!("Loading texture: {:?}", texture_id);
                    if let Some(texture) = texture {
                        bind_group_layouts.push(texture.get_bind_group_layout());
                    }
                }
                _ => {}
            }
        }

        let pipeline = Pipeline::new(self._device.clone(), shader_module.unwrap(),
                                     bind_group_layouts, vertex_buffer_layouts, self.use_depth);

        self.pipeline = Some(pipeline);
    }

    pub(super) fn execute(&self, id: usize, texture_view: &wgpu::TextureView,
                          resource_manager: &mut ResourceManager, encoder: &mut wgpu::CommandEncoder) {
        if let Some(pipeline) = &self.pipeline {

            let depth_texture = resource_manager.load_depth_texture();

            let depth_texture_locked = depth_texture.lock().unwrap();

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
                        view: &depth_texture_locked.view,
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

            if let Some(static_uniform_set) = &self.static_uniform_set {
                static_uniform_set.bind(0, &mut render_pass);
            }

            if let Some(dynamic_uniform_set) = &self.dynamic_uniform_set {
                dynamic_uniform_set.bind(1, &mut render_pass);
            }


            for command in self.commands.iter() {
                match command {
                    Command::DrawMesh(mesh_id) => {
                        let mesh = resource_manager.get_mesh(mesh_id.clone());

                        if let Some(mesh) = mesh {
                            mesh.render(&mut render_pass);
                        }
                    }
                    Command::DrawMeshInstanced(mesh_id, count, _) => {
                        let mesh = resource_manager.get_mesh(mesh_id.clone());

                        if let Some(mesh) = mesh {
                            mesh.render_instanced(&mut render_pass);
                        }
                    }
                    Command::BindTexture(index, texture_id) => {
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
