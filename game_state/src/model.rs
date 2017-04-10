use Renderable;
use Identifyable;

use std::path::Path;

use cgmath::SquareMatrix;
use cgmath::Matrix4;

#[derive(Debug)]
pub struct Material { }

#[derive(Debug)]
pub struct Model {
    pub filename: String,
    pub id: u64,
    pub model_mat: Matrix4<f32>,
    pub world_mat: Matrix4<f32>,
    pub material: Material,
    pub mesh: Mesh,
}

use std::sync::atomic::{ AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
static GLOBAL_MODEL_ID: AtomicUsize = ATOMIC_USIZE_INIT;

impl Model {
    pub fn create(filename: &str, model_mat: Matrix4<f32>) -> Self {
/*
        let vectors = o.vertices.windows(3);
        let normals = o.normals.windows(3);

        let data = if normals.len() > 0 {
            vectors.zip(normals).map(|(v,n)| {
                Vertex::from(Vector( v[0], v[1], v[2] ), Normal( n[0], n[1], n[2]))
            }).collect::<Vec<_>>()
        } else {
            vectors.map(|v| {
                Vertex::from(Vector( v[0], v[1], v[2] ), Normal(1.0, 1.0, 1.0))
            }).collect::<Vec<_>>()
        };

        assert!(data.len() > 0);

*/
        let build = Model {
            filename: filename.to_string(),
            id: 0,//GLOBAL_MODEL_ID.fetch_add(1, Ordering::SeqCst) as u64,
            model_mat: model_mat,
            world_mat: Matrix4::<f32>::identity(),
            //mesh: Mesh::create(Vec::new(), Vec::new(), Vec::new()),
            mesh: Mesh::create(Vec::new(), Vec::new()),//data, o.indices.iter().map(|x:&u32| *x as u16).collect::<Vec<_>>()),
            material: Material {},
        };
        println!("Loaded model: \n{:?}", build);
        build
    }
}

#[test]
fn load_teapot_obj() {
    let model = Model::create("teapot.obj", Matrix4::<f32>::identity());
    assert_eq!(model.mesh.vertices.len(), 42);
}

#[test]
fn slice_windows_learning() {

    let vec1 = [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec2 = [10,11,12,13,14,15,16,17,18,19];
    let vec3 = [10,11,12,13,14,15,16];

    let one = vec1.windows(3);
    let two = vec2.windows(3);
    let three = vec3.windows(2);

    for ((a, b), c) in one.zip(two).zip(three) {
        assert_eq!(a.len(), 3);
        assert_eq!(b.len(), 3);
        assert_eq!(c.len(), 2);
    }


    //let model = Model::create("assets/models/teapot.obj", Matrix4::<f32>::identity());

}

#[derive(Debug, Copy, Clone)] pub struct Vector(pub f32,pub f32,pub f32);
#[derive(Debug, Copy, Clone)] pub struct Normal(pub f32,pub f32,pub f32);
#[derive(Debug, Copy, Clone)] pub struct Vertex {
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

#[derive(Debug)]
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
