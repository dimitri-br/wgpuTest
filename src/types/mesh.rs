use crate::types::{Instance, Vertex};
use wgpu::util::{DeviceExt, DrawIndirectArgs, DrawIndexedIndirectArgs};
use crate::Handle;

use super::InstanceBuffer;

pub struct Mesh {
    submeshes: Vec<Submesh>,
    instance_buffer: Option<InstanceBuffer>
}

impl Mesh {
    pub fn load_from_file<T: AsRef<std::path::Path> + std::fmt::Debug>(
        device: Handle<wgpu::Device>,
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

            submeshes.push(Submesh::new(&device, &vertices, indices));
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

    pub fn set_instances(&mut self, device: &wgpu::Device, instance_buffer: InstanceBuffer) {
        for submesh in self.submeshes.iter_mut(){
            submesh.create_instanced_indirect_args(device, &instance_buffer);
        }
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
    indirect_args: Option<wgpu::Buffer>,
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
            indirect_args: None,
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

        if let Some(indirect_args) = &self.indirect_args{
            render_pass.draw_indexed_indirect(indirect_args, 0);
        }else{
            render_pass.draw_indexed(0..self.num_indices, 0, 0..instance_buffer.get_instance_count() as u32);
        }
    }

    fn create_instanced_indirect_args(&mut self, device: &wgpu::Device, instance_buffer: &InstanceBuffer){
        let indirect_args = DrawIndexedIndirectArgs {
            index_count: self.num_indices as u32,
            instance_count: instance_buffer.get_instance_count() as u32,
            first_index: 0,
            base_vertex: 0,
            first_instance: 0,
        };

        // cast to u8; note this is not a bytemuck cast as the struct is not repr(C)
        let indirect_args_data = {
            let indirect_args_ptr = &indirect_args as *const DrawIndexedIndirectArgs as *const u8;
            unsafe { std::slice::from_raw_parts(indirect_args_ptr, std::mem::size_of::<DrawIndexedIndirectArgs>()) }
        };
        

        let indirect_args_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Indirect Args Buffer"),
            contents: indirect_args_data,
            usage: wgpu::BufferUsages::INDIRECT,
        });

        self.indirect_args = Some(indirect_args_buffer);
    }
}
