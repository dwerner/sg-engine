use std::error::Error;
use std::path::Path;

use nalgebra::Matrix4;

// TODO: still need to refactor nom-obj to take BufReader, among other things
use nom_obj::model::{Interleaved, Obj};

use crate::create_next_identity;
use crate::Identifyable;
use crate::Identity;

#[derive(Clone)]
pub struct Material {
    pub diffuse_map: image::DynamicImage,
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
    pub fn load(filename: &str, model_mat: Matrix4<f32>) -> Result<Vec<Self>, Box<dyn Error>> {
        let obj = Obj::read_file(filename)?;

        let mut models = Vec::new();
        for o in obj.objects.iter() {
            let Interleaved { v_vt_vn, idx } = o.interleaved();

            let verts = v_vt_vn
                .iter()
                .map(|&(v, vt, vn)| Vertex::new((v.0, v.1, v.2), vt, (vn.0, vn.1, vn.0)))
                .collect::<Vec<_>>();

            if verts.is_empty() {
                return Err("model has no vertices".into());
            }

            let indices = idx.iter().map(|x: &usize| *x as u16).collect::<Vec<_>>();

            let diffuse_map_filename = &obj.objects[0]
                .material
                .as_ref()
                .ok_or_else(|| format!("no diffuse map (map_Kd) defined in model {}", filename))?
                .diffuse_map;

            let material_path = Path::new(diffuse_map_filename);
            let diffuse_map = image::open(material_path)?;

            models.push(Model {
                filename: filename.to_string(),
                id: create_next_identity(),
                model_mat,
                world_mat: Matrix4::<f32>::identity(),
                mesh: Mesh::create(verts, indices),
                material: Material { diffuse_map },
            })
        }

        Ok(models)
    }
}

#[test]
#[cfg(test)]
fn slice_windows_learning() {
    let vec1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec2 = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let vec3 = [10, 11, 12, 13, 14, 15, 16];

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

#[derive(Debug, Copy, Clone)]
pub struct Vector(pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct UVW(pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct Normal(pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector,
    pub uvw: UVW,
    pub normal: Normal,
}

impl Vertex {
    pub fn new(v: (f32, f32, f32), vt: (f32, f32, f32), vn: (f32, f32, f32)) -> Self {
        Vertex {
            position: Vector(v.0, v.1, v.2),
            uvw: UVW(v.0, vt.1, vt.2),
            normal: Normal(vn.0, vn.1, vn.2),
        }
    }

    pub fn from_parts(v: Vector, u: UVW, n: Normal) -> Self {
        Vertex {
            position: v,
            uvw: u,
            normal: n,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn create(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Mesh { vertices, indices }
    }
}

impl Identifyable for Model {
    fn identify(&self) -> u64 {
        self.id
    }
}
