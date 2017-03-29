use game_state::model::{ Vertex as GSVertex };

#[derive(Debug, Clone)]
pub struct Vertex {
    position: [f32;3],
    normal: [f32;3]
}

impl_vertex!(Vertex, position, normal);
