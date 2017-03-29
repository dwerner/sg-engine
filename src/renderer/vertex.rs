use game_state::model::{ Vertex as GSVertex };

use std::convert::From;

#[derive(Debug, Clone)]
pub struct Vertex {
    position: [f32;3],
    normal: [f32;3]
}

impl From<GSVertex> for Vertex {
    fn from(g: GSVertex) -> Self {
        Vertex{
            position:[g.position.0, g.position.1, g.position.2],
            normal:[g.normal.0, g.normal.1, g.normal.2]
        }
    }
}


impl_vertex!(Vertex, position, normal);
