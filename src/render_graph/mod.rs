mod render_node;
mod commands;
mod resource_manager;

pub use render_node::RenderNode;
pub use commands::{Command, DrawCommand};
pub use resource_manager::{ResourceManager, ResourceHandle, ResourceType};
use crate::MutHandle;

pub struct RenderGraph{
    nodes: Vec<RenderNode>,
}

impl RenderGraph{
    pub fn new() -> Self{
        Self{
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: RenderNode){
        self.nodes.push(node);
    }

    pub fn build(&mut self, resource_manager: MutHandle<ResourceManager>){
        for node in self.nodes.iter_mut(){
            node.build_pipeline(resource_manager.clone());
        }
    }

    pub fn execute(&self, texture_view: &wgpu::TextureView, resource_manager: MutHandle<ResourceManager>, encoder: &mut wgpu::CommandEncoder){
        for (id, node) in self.nodes.iter().enumerate(){
            node.execute(id, texture_view, resource_manager.clone(), encoder);
        }
    }
}