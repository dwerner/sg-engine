#[derive(Debug, Clone)]
pub struct GVertex {
    pub position: [f32;3],
    pub color: [f32;4]
}

impl GVertex {
    pub fn new(position: [f32;3], color: [f32;4] ) -> Self {
        GVertex { position: position, color: color}
    }
}

pub struct Model {
    pub vertices: Vec<GVertex>,
    pub indices: Vec<usize>
}

pub struct Material {

}
