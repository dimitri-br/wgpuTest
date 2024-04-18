#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transform{
    pub position: [f32; 4],
    pub rotation: [f32; 4],
    pub scale: [f32; 4],
}

impl Transform{
    pub fn new(position: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) -> Self{
        Self{
            position: [position[0], position[1], position[2], 0.0],
            rotation: [rotation[0], rotation[1], rotation[2], 0.0],
            scale: [scale[0], scale[1], scale[2], 0.0],
        }
    }
}