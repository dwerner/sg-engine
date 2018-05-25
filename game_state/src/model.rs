use Renderable;
use Identity;
use Identifyable;
use create_next_identity;

use cgmath::SquareMatrix;
use cgmath::Matrix4;
use image;

use nom_obj::model::{
    Obj,
    Interleaved
};

#[derive(Clone)]
pub struct Material {
    pub diffuse_map: image::DynamicImage
}

#[derive(Clone)]
pub struct Model {
    pub filename: String,
    pub id: Identity,
    pub model_mat: Matrix4<f32>,
    pub world_mat: Matrix4<f32>,
    pub material: Material,
    pub mesh: Mesh,
}

impl Model {
    pub fn create(filename: &str, model_mat: Matrix4<f32>) -> Self {

        let obj = Obj::create(filename);
        let Interleaved{ v_vt_vn, idx } = obj.objects[0].interleaved();

        let verts = v_vt_vn.iter()
            .map(|&(v,vt,vn)| Vertex::create(v.0, v.1, v.2, vt.0, vt.1, vt.2, vn.0, vn.1, vn.0) )
            .collect::<Vec<_>>();

        assert!(verts.len() > 0);

        let indices = idx.iter()
            .map(|x:&usize| *x as u16)
            .collect::<Vec<_>>();

        use std::path::Path;
        let path_str = obj.get_mtl().diffuse_map.clone();
        let material_path = Path::new(&path_str);
        let diffuse_map = image::open(material_path).expect("unable to open image file from material");

        let build = Model {
            filename: filename.to_string(),
            id: create_next_identity(),
            model_mat: model_mat,
            world_mat: Matrix4::<f32>::identity(),
            mesh: Mesh::create(verts, indices),
            material: Material {
                diffuse_map: diffuse_map
            },
        };
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
#[derive(Debug, Copy, Clone)] pub struct UVW(pub f32,pub f32,pub f32);
#[derive(Debug, Copy, Clone)] pub struct Normal(pub f32,pub f32,pub f32);
#[derive(Debug, Copy, Clone)] pub struct Vertex {
    pub position: Vector,
    pub uvw: UVW,
    pub normal: Normal
}

impl Vertex {
    pub fn create(vx:f32, vy:f32, vz:f32, u:f32, v:f32, w:f32, nx:f32, ny:f32, nz:f32) -> Self {
        Vertex {
            position: Vector(vx,vy,vz),
            uvw: UVW(u,v,w),
            normal: Normal(nx,ny,nz)
        }
    }

    pub fn from(v: Vector, u:UVW, n: Normal) -> Self {
        Vertex{ position: v, uvw: u, normal: n }
    }
}

#[derive(Clone, Debug)]
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
    fn get_diffuse_map(&self) -> &image::DynamicImage {
        &self.material.diffuse_map
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
