use crate::types;

pub enum Command{
    LoadShader(String),

    // Draw commands

    // Bind a texture to a texture unit (will also load the texture if it's not loaded)
    BindTexture(u32, String),

    // Draw a mesh (will also load the mesh if it's not loaded)
    DrawMesh(String),
    // Draw an instanced mesh (will also load the mesh if it's not loaded)
    DrawMeshInstanced(String, u32, Vec<types::Transform>),
}