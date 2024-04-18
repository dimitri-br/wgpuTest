use crate::types;

pub enum Command{
    LoadShader(String),
    LoadTexture(String),
    CreateMaterial(String),

    // Draw commands
    BindTexture(u32, String),
    DrawMesh(String),
    DrawMeshInstanced(String, u32, Vec<types::Transform>),
}