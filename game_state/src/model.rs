use Renderable;
use Identifyable;

use cgmath::SquareMatrix;
use cgmath::Matrix4;

pub struct Material { }

pub struct Model {
    pub filename: String,
    pub id: u64,
    pub model_mat: Matrix4<f32>,
    pub world_mat: Matrix4<f32>,
    pub material: Material,
    pub mesh: Mesh,
}

impl Model {
    pub fn create(filename: &str, model_mat: Matrix4<f32>) -> Self {
        Model {
            filename: filename.to_string(),
            id: 0, // u64 id
            model_mat: model_mat,
            world_mat: Matrix4::<f32>::identity(),
            //mesh: Mesh::create(Vec::new(), Vec::new(), Vec::new()),
            mesh: Mesh::create(vec![
                Vertex::from(Vector( 0.0, 0.9, 0.0),  Normal( 0.0, 0.0, -1.0)),
                Vertex::from(Vector( 0.5, 0.0, 0.0),  Normal( 0.0, 0.0, -1.0)),
                Vertex::from(Vector( 0.0, 0.0, 0.5),  Normal( 0.0, 0.0, -1.0)),
                Vertex::from(Vector( 0.0, 0.0, -0.5), Normal( 0.0, 0.0, -1.0)),
                Vertex::from(Vector( -0.5, 0.0, 0.0), Normal( 0.0, 0.0, -1.0)),
            ], vec![
                0u16, 1, 2,
                0,1,3,
                0,1,4,
            ]),
            material: Material {},
        }
    }
}

#[derive(Copy, Clone)] pub struct Vector(pub f32,pub f32,pub f32);
#[derive(Copy, Clone)] pub struct Normal(pub f32,pub f32,pub f32);
#[derive(Copy, Clone)] pub struct Vertex {
    pub position: Vector,
    pub normal: Normal
}

impl Vertex {
    pub fn create(vx:f32, vy:f32, vz:f32, nx:f32, ny:f32, nz:f32) -> Self {
        Vertex {
            position: Vector(vx,vy,vz),
            normal: Normal(nx,ny,nz)
        }
    }

    pub fn from(v: Vector, n: Normal) -> Self {
        Vertex{ position:v, normal:n }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn create(v: Vec<Vertex>, i: Vec<u16>) -> Self {
        Mesh { vertices:v, indices:i }
    }
}

impl Renderable for Model {
    fn get_mesh(&self) -> &Mesh { &self.mesh }

    fn get_world_matrix(&self) -> &Matrix4<f32> {
        &self.world_mat
    }

    fn get_model_matrix(&self) -> &Matrix4<f32> {
        &self.model_mat
    }

    fn set_world_matrix(&mut self, mat: Matrix4<f32>) {
        self.world_mat = mat;
    }
    fn set_model_matrix(&mut self, mat: Matrix4<f32>) {
        self.model_mat = mat;
    }
}

impl Identifyable for Model {
    fn identify(&self) -> u64 { self.id }
}
