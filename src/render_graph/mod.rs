mod render_node;
mod commands;
mod resource_manager;

pub use render_node::RenderNode;
pub use commands::Command;
pub use resource_manager::ResourceManager;

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

    pub fn build(&mut self, resource_manager: &mut ResourceManager){
        for node in self.nodes.iter_mut(){
            node.build_pipeline(resource_manager);
        }
    }

    pub fn execute(&self, texture_view: &wgpu::TextureView, resource_manager: &ResourceManager, encoder: &mut wgpu::CommandEncoder){
        for node in self.nodes.iter(){
            node.execute(texture_view, resource_manager, encoder);
        }
    }
}