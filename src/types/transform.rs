use crate::types::Instance;

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

    pub fn to_instance(&self) -> Instance{
        Instance{
            model: self.to_matrix(),
        }
    }

    pub fn to_matrix(&self) -> [[f32; 4]; 4]{
        let mut matrix = nalgebra::Matrix4::identity();
        matrix *= nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(self.position[0], self.position[1], self.position[2]));
        matrix *= nalgebra::Matrix4::from_euler_angles(self.rotation[0], self.rotation[1], self.rotation[2]);
        matrix *= nalgebra::Matrix4::new_nonuniform_scaling(&nalgebra::Vector3::new(self.scale[0], self.scale[1], self.scale[2]));
        matrix.into()
    }
}