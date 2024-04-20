use crate::render_graph::ResourceHandle;
use crate::types;

pub enum Command{
    LoadShader(String),

    // Draw commands

    // Bind a texture to a texture unit (will also load the texture if it's not loaded)
    BindTexture(u32, String),

    // Draw a mesh (will also load the mesh if it's not loaded)
    DrawMesh(String),
    // Draw an instanced mesh (will also load the mesh if it's not loaded)
    DrawMeshInstanced(String, Vec<types::Transform>),
}

pub enum DrawCommand {
    // These are the commands we use to execute the render graph
    // They have the ResourceID of the resource they are using,
    // and any other relevant data
    BindTexture(u32, ResourceHandle),

    DrawMesh(ResourceHandle),

    DrawMeshInstanced(ResourceHandle),
}