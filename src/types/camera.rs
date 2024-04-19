use encase::{ShaderType, UniformBuffer};
use log::info;

use crate::MutHandle;

use super::Uniform; // Encase is a library that ensures that the uniforms are correctly aligned for the shader


#[derive(Debug, Clone)]
pub struct Camera{
    pub position: nalgebra::Vector3<f32>,
    pub rotation: nalgebra::Vector3<f32>,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    forward: nalgebra::Vector3<f32>,
    right: nalgebra::Vector3<f32>,
    up: nalgebra::Vector3<f32>,
}

impl Camera{
    pub fn new(position: nalgebra::Vector3<f32>, rotation: nalgebra::Vector3<f32>, fov: f32, surface: MutHandle<wgpu::SurfaceConfiguration>) -> Self{
        let surface = surface.lock().unwrap();
        let aspect = surface.width as f32 / surface.height as f32;

        let near = 0.1;
        let far = 100.0;

        Self{
            position,
            rotation,
            fov,
            aspect,
            near,
            far,

            forward: nalgebra::Vector3::new(-180.0, 0.0, 1.0),
            right: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            up: nalgebra::Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub(crate) fn to_wgpu_bytes(&self) -> Vec<u8>{
        let mut buffer = UniformBuffer::new(Vec::new());
        buffer.write(&CameraUniform::new(self)).unwrap();
        buffer.into_inner()
    }

    pub fn update(&mut self){
        // Calculate the target based on the rotation
        let forward = nalgebra::Vector3::new(
            self.rotation[0].to_radians().cos() * self.rotation[1].to_radians().cos(),
            self.rotation[1].to_radians().sin(),
            self.rotation[1].to_radians().cos() * self.rotation[0].to_radians().sin(),
        );

        let right =  forward.cross(&nalgebra::Vector3::new(0.0, 1.0, 0.0)).normalize();

        self.forward = forward;
        self.right = right;
        self.up = right.cross(&forward).normalize();

    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>){
        self.aspect = size.width as f32 / size.height as f32;
        self.update();
    }

    pub fn move_position(&mut self, delta: nalgebra::Vector3<f32>){
        // Offset the position based on the camera's forward and right vectors
        let delta = self.forward * delta[2] + self.right * delta[0] + self.up * delta[1];
        self.position += delta;
        self.update();
    }

    pub fn move_rotation(&mut self, delta: nalgebra::Vector3<f32>){
        // Limit the rotation to avoid gimbal lock
        if self.rotation[1] + delta[1] > 89.9{
            self.rotation[1] = 89.9;
        }else if self.rotation[1] + delta[1] < -89.9{
            self.rotation[1] = -89.9;
        }else{
            self.rotation += delta;
        }
        self.update();
    }
}

impl Default for Camera{
    fn default() -> Self{
        Self{
            position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            fov: 45.0,
            aspect: 1.0,
            near: 0.1,
            far: 100.0,

            forward: nalgebra::Vector3::new(-90.0, 0.0, -1.0),
            right: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            up: nalgebra::Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Uniform for Camera{
    fn to_wgpu(&self) -> Vec<u8>{
        self.to_wgpu_bytes()
    }
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct CameraUniform{
    view_proj: nalgebra::Matrix4<f32>,
}

impl CameraUniform{
    pub fn new(camera: &Camera) -> Self{
        let view = nalgebra::Matrix4::look_at_rh(&camera.position.into(), &(camera.position + camera.forward).into(), &camera.up.into());

        let proj = nalgebra::Perspective3::new(camera.aspect, camera.fov.to_radians(), camera.near, camera.far).into_inner();

        let view_proj = proj * view;

        Self{
            view_proj,
        }
    }
}
