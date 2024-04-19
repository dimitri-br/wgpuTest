use crate::types::{Instance, Vertex};
use wgpu::util::DeviceExt;

use super::InstanceBuffer;

pub struct Mesh {
    submeshes: Vec<Submesh>,
    instance_buffer: Option<InstanceBuffer>
}

impl Mesh {
    pub fn load_from_file<T: AsRef<std::path::Path> + std::fmt::Debug>(
        device: &wgpu::Device,
        path: T,
    ) -> Self {
        let load_options = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        };

        let (meshes, _) = tobj::load_obj(path, &load_options).unwrap();

        let mut submeshes = Vec::new();

        for mesh in meshes.iter() {
            let positions: Vec<[f32; 3]> = mesh
                .mesh
                .positions
                .chunks(3)
                .map(|c| -> [f32; 3] {
                    assert_eq!(c.len(), 3);
                    [c[0], c[1], c[2]]
                })
                .collect();

            let normals: Vec<[f32; 3]> = mesh
                .mesh
                .normals
                .chunks(3)
                .map(|c| -> [f32; 3] {
                    assert_eq!(c.len(), 3);
                    [c[0], c[1], c[2]]
                })
                .collect();

            let tex_coords: Vec<[f32; 2]> = mesh
                .mesh
                .texcoords
                .chunks(2)
                .map(|c| -> [f32; 2] {
                    assert_eq!(c.len(), 2);
                    [c[0], c[1]]
                })
                .collect();

            let vertices: Vec<Vertex> = positions
                .iter()
                .zip(normals.iter().zip(tex_coords.iter()))
                .map(|(pos, tex_norms)| Vertex {
                    position: *pos,
                    normal: *tex_norms.0,
                    tex_coords: *tex_norms.1,
                })
                .collect();

            let indices = &mesh.mesh.indices;

            submeshes.push(Submesh::new(device, &vertices, indices));
        }

        Self {
            submeshes,
            instance_buffer: None
        }
    }

    pub fn load_from_raw(device: &wgpu::Device, vertices: &[Vertex], indices: &[u32]) -> Self {
        // Create a submesh
        let submesh = Submesh::new(device, vertices, indices);

        Self {
            submeshes: vec![submesh],
            instance_buffer: None
        }
    }

    pub fn set_instances(&mut self, instance_buffer: InstanceBuffer) {
        self.instance_buffer = Some(instance_buffer);
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for submesh in self.submeshes.iter() {
            submesh.render(render_pass);
        }
    }

    pub fn render_instanced<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>){
        if let Some(ib) = &self.instance_buffer{
            for submesh in self.submeshes.iter(){
                submesh.render_instanced(render_pass, &ib);
            }
        }
    }
}

pub struct Submesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Submesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u32]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn render_instanced<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, instance_buffer: &'a InstanceBuffer){
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        instance_buffer.bind_as_buffer(1, render_pass);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..instance_buffer.get_instance_count() as u32);
    }
}
