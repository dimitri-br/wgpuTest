use crate::render_graph::ResourceHandle;
use crate::Transform;

pub struct RenderObject{
    pub mesh: ResourceHandle,
    pub material: ResourceHandle,
    pub transform: Transform
}