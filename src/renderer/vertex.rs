use game_state::model::{ GVertex, Vector, Normal as GNormal };

// TODO: determine the best course of action for Vertex, as this is shader input.
#[derive(Debug, Clone)]
pub struct Vertex {
    position: [f32;3]
}
// TODO: determine the best course of action for Vertex, as this is shader input.
#[derive(Debug, Clone)]
pub struct Normal {
    normal: [f32;3]
}

impl Normal {
    pub fn from_normal(normal: GNormal) -> Self {
        Normal{ normal: [normal.0, normal.1, normal.2] }
    }
}

impl Vertex {
    pub fn new(position: [f32;3]) -> Self {
        Vertex { position: position}
    }
    pub fn from_gvertex(vert: GVertex) -> Self { // implies copying :(
        Vertex { position: vert.position }
    }
    pub fn from_vector(vector: Vector) -> Self {
        Vertex{ position: [vector.0, vector.1, vector.2], }
    }
}

impl_vertex!(Vertex, position); // passing arguments to shaders here
impl_vertex!(Normal, normal);
