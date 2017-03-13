use game_state::model::{ GVertex };

// TODO: determine the best course of action for Vertex, as this is shader input.
#[derive(Debug, Clone)]
pub struct Vertex {
    position: [f32;3],
    color: [f32;4]
}

impl Vertex {
    pub fn new(position: [f32;3], color: [f32;4] ) -> Self {
        Vertex { position: position, color: color}
    }
    pub fn from(vert: GVertex) -> Self { // implies copying :(
        Vertex { position: vert.position, color: vert.color }
    }
}

impl_vertex!(Vertex, position, color); // passing arguments to shaders here
