use encase::ShaderType;

use crate::types::Instance;

use super::Uniform;

#[derive(Debug, Clone, Copy)]
pub struct Transform{
    pub position: nalgebra::Vector3<f32>,
    pub rotation: nalgebra::Vector3<f32>,
    pub scale: nalgebra::Vector3<f32>,
}

impl Transform{
    pub fn new(position: nalgebra::Vector3<f32>, rotation: nalgebra::Vector3<f32>, scale: nalgebra::Vector3<f32>) -> Self{
        Self{
            position,
            rotation,
            scale,
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

impl Uniform for Transform{
    fn to_wgpu(&self) -> Vec<u8>{
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(&TransformUniform::new(self)).unwrap();
        buffer.into_inner()
    }
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct TransformUniform{
    pub model: nalgebra::Matrix4<f32>,
}

impl TransformUniform{
    pub fn new(transform: &Transform) -> Self{
        Self{
            model: transform.to_matrix().into(),
        }
    }
}