use Renderable;
use Identifyable;

use cgmath::SquareMatrix;
use cgmath::Matrix4;

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

pub struct Material { }

pub struct Model {
    pub id: u64,
    pub view_mat: Matrix4<f32>,
    pub world_mat: Matrix4<f32>,
    pub material: Material,
    pub mesh: Mesh,
}

impl Model {
    pub fn create(filename: &str) -> Self {
        Model {
            id: 0, // u64 id
            view_mat: Matrix4::<f32>::identity(),
            world_mat: Matrix4::<f32>::identity(),
            //mesh: Mesh::create(Vec::new(), Vec::new(), Vec::new()),
            mesh: Mesh::create(vec![
                Vector(-0.5,-0.5, 0.0),
                Vector( 0.5,-0.5, 0.0),
                Vector( 0.5, 0.5, 0.0),
            ], vec![
                Normal( 0.0, 0.0, 1.0),
                Normal( 0.0, 0.0, 1.0),
                Normal( 0.0, 0.0, 1.0),
            ], vec![
                0u16, 1, 2
            ]),
            material: Material {},
        }
    }
}

#[derive(Copy, Clone)] pub struct Vector(pub f32,pub f32,pub f32);
#[derive(Copy, Clone)] pub struct Normal(pub f32,pub f32,pub f32);

pub struct Mesh {
    pub vertices: Vec<Vector>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn create(v: Vec<Vector>, n: Vec<Normal>, i: Vec<u16>) -> Self {
        assert!(v.len() == i.len()); // can we statically protect for sizing?
        Mesh { vertices:v, normals:n, indices:i }
    }
}

impl Renderable for Model {
    fn get_mesh(&self) -> &Mesh { &self.mesh }

    fn get_view_matrix(&self) -> &Matrix4<f32> {
        &self.view_mat
    }

    fn get_world_matrix(&self) -> &Matrix4<f32> {
        &self.world_mat
    }
}

impl Identifyable for Model {
    fn identify(&self) -> u64 { self.id }
}
