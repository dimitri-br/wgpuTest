
use std::collections::HashMap;
use crate::Handle;

use crate::types::{Mesh, Texture};

type ResourceID = String;

pub struct ResourceManager{
    meshes: HashMap<ResourceID, Mesh>,
    textures: HashMap<ResourceID, Texture>,

    device: Handle<wgpu::Device>,
    queue: Handle<wgpu::Queue>,
}


impl ResourceManager{
    pub fn new(device: Handle<wgpu::Device>, queue: Handle<wgpu::Queue>) -> Self {
        Self{
            meshes: HashMap::new(),
            textures: HashMap::new(),
            device,
            queue
        }
    }

    pub fn load_mesh<T>(&mut self, id: ResourceID, path: T) -> Option<&Mesh> where T: AsRef<std::path::Path> + std::fmt::Debug{
        // Check if the mesh already exists
        if self.meshes.contains_key(&id){
            return self.meshes.get(&id);
        }

        // Load the mesh
        let mesh = Mesh::load_from_file(&self.device, path);
        self.meshes.insert(id.clone(), mesh);

        self.meshes.get(&id)
    }

    pub fn load_texture<T>(&mut self, id: ResourceID, path: T) -> Option<&Texture> where T: AsRef<std::path::Path>{
        // Check if the texture already exists
        if self.textures.contains_key(&id){
            return self.textures.get(&id);
        }

        // Load the texture
        let texture = Texture::load_from_path(&self.device, &self.queue, path);
        self.textures.insert(id.clone(), texture);

        self.textures.get(&id)
    }

    pub fn get_mesh(&self, id: ResourceID) -> Option<&Mesh>{
        self.meshes.get(&id)
    }

    pub fn get_texture(&self, id: ResourceID) -> Option<&Texture>{
        self.textures.get(&id)
    }
}