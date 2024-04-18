mod device_handler;
mod instance_handler;
mod render_graph;
mod renderer;
mod surface_wrapper;
mod types;
mod pipeline;

pub use renderer::Renderer;

pub use render_graph::Command;
pub use types::UniformBufferType;

use std::sync::{Arc, Mutex};

pub type Handle<T> = Arc<T>;
pub type MutHandle<T> = Handle<Mutex<T>>;
