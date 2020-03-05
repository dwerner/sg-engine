use game_state::model::Vertex as GSVertex;

#[derive(Default, Debug, Clone)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

impl Vertex {
    pub fn from_gs_vertex(g: GSVertex) -> Self {
        println!("vertex {:?}", g);
        Vertex {
            position: [g.position.0, g.position.1, g.position.2],
            uv: [g.uvw.0, g.uvw.1],
            normal: [g.normal.0, g.normal.1, g.normal.2],
        }
    }
}

// the reason for this copying is to put the data into a struct we can
// impl_vertex! on, in this crate...
vulkano::impl_vertex!(Vertex, position, normal, uv);
