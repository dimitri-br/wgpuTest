mod device_handler;
mod instance_handler;
mod render_graph;
mod renderer;
mod surface_wrapper;
mod types;
mod pipeline;

pub use renderer::Renderer;

pub use render_graph::Command;
pub use types::Camera;
pub use types::Transform;
pub use types::UniformBufferType;

use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

//pub type Handle<T> = Arc<T>;
//pub type MutHandle<T> = Handle<Mutex<T>>;

pub struct Handle<T>{
    inner: Arc<T>,
}

impl<T> Handle<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct MutHandle<T>{
    inner: Handle<Mutex<T>>,
}

impl<T> MutHandle<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Handle::new(Mutex::new(inner)),
        }
    }

    pub fn lock(&self) -> std::sync::LockResult<std::sync::MutexGuard<T>> {
        self.inner.lock()
    }

    pub fn borrow<'a>(&'a self) -> std::sync::MutexGuard<T> {
        self.inner.lock().unwrap()
    }
}

impl<T> Clone for MutHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for MutHandle<T> {
    type Target = Handle<Mutex<T>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for MutHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


