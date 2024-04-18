
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::{Handle, MutHandle};

use crate::types::{Mesh, Texture};

type ResourceID = String;

pub struct ResourceManager{
    meshes: HashMap<ResourceID, Mesh>,
    textures: HashMap<ResourceID, Texture>,

    // Renderer Resources
    depth_texture: Option<MutHandle<Texture>>,

    device: Handle<wgpu::Device>,
    queue: Handle<wgpu::Queue>,
    surface_configuration: MutHandle<wgpu::SurfaceConfiguration>
}


impl ResourceManager{
    pub fn new(device: Handle<wgpu::Device>, queue: Handle<wgpu::Queue>,
               surface_configuration: MutHandle<wgpu::SurfaceConfiguration>) -> Self {
        Self{
            meshes: HashMap::new(),
            textures: HashMap::new(),

            depth_texture: None,

            surface_configuration,

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

    pub fn load_depth_texture(&mut self) -> MutHandle<Texture>{
        if self.depth_texture.is_some(){

            let mut isWidth = false;
            let mut isHeight = true;

            // Define a closure to check if the depth texture has the same size as the surface configuration
            {
                let surface_config = self.surface_configuration.clone();

                let surface_config = surface_config.lock().unwrap();

                let texture_size = self.depth_texture
                    .as_ref().unwrap()
                    .lock().unwrap()
                    .get_texture_size();

                isWidth = texture_size.width == surface_config.width;
                isHeight = texture_size.height == surface_config.height;
            }

            // Check if the depth texture has the same size as the surface configuration
            if !isWidth || !isHeight{
                self.depth_texture.as_ref().unwrap()
                    .lock().unwrap()
                    .resize_screen_texture(&self.device, self.surface_configuration.clone());
            }

            return self.depth_texture.as_ref().unwrap().clone();
        }

        let texture = Texture::create_depth_texture(&self.device, self.surface_configuration.clone());
        self.depth_texture = Some(Arc::new(Mutex::new(texture)));

        self.depth_texture.as_ref().unwrap().clone()
    }

    pub fn get_mesh(&self, id: ResourceID) -> Option<&Mesh>{
        self.meshes.get(&id)
    }

    pub fn get_texture(&self, id: ResourceID) -> Option<&Texture>{
        self.textures.get(&id)
    }
}