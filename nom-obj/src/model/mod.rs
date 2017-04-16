use parser::obj::{
    ObjParser,
    ObjLine,
    FaceIndex,
};

use parser::mtl::{
    MtlLine,
    MtlParser
};

pub struct Obj {
    comments: Vec<ObjLine>,
    objects: Vec<ObjObject>,
}
impl Obj {
    pub fn create(filename: &'static str) -> Self {
        let mut obj = Obj{
            comments: Vec::new(),
            objects: Vec::new()
        };
        let parser = ObjParser::create(filename);

        let mut object = ObjObject::new();
        for line in parser {
            match line {
                ObjLine::ObjectName(name) => {
                    // new object encountered, when multiple objects exist
                    if object.name.is_some() {
                        obj.objects.push(object);
                        object = ObjObject::new();
                    }
                    object.name = Some(name);
                },
                ObjLine::MtlLib(name) => object.material = Some(name),
                ObjLine::Vertex(..) => object.vertices.push(line),
                ObjLine::VertexParam(..) => object.vertex_params.push(line),
                ObjLine::Face(..) => object.faces.push(line),
                ObjLine::Normal(..) => object.normals.push(line),
                ObjLine::TextureUVW(..) => object.normals.push(line),
                _ => {}
            }
        }
        obj.objects.push(object);
        obj
    }
}

#[derive(Debug)]
pub struct ObjObject {
    name: Option<String>,
    material: Option<String>,
    vertices: Vec<ObjLine>,
    normals: Vec<ObjLine>,
    texture_coords: Vec<ObjLine>,
    vertex_params: Vec<ObjLine>,
    faces: Vec<ObjLine>,
}

impl ObjObject {
    pub fn new() -> Self {
        ObjObject{
            name: None,
            material: None,
            vertices: Vec::new(),
            normals: Vec::new(),
            texture_coords: Vec::new(),
            vertex_params: Vec::new(),
            faces: Vec::new(),
        }
    }
    pub fn vertices(&self) -> &Vec<ObjLine> { &self.vertices }
    pub fn vertex_params(&self) -> &Vec<ObjLine> { &self.vertex_params }
    pub fn normals(&self) -> &Vec<ObjLine> { &self.normals }
    pub fn texture_coords(&self) -> &Vec<ObjLine> { &self.texture_coords }
    pub fn material(&self) -> &Option<String> { &self.material }
    pub fn name(&self) -> &Option<String> { &self.name }
    pub fn faces(&self) -> &Vec<ObjLine> { &self.faces }

    pub fn interleaved(&self) -> Interleaved {
        let mut data = Interleaved{ v_vt_vn: Vec::new(), idx:Vec::new() };
        for i in 0..&self.faces.len()-1 {
            match &self.faces[i] {
                &ObjLine::Face(
                    FaceIndex(v1, vt1, vn1),
                    FaceIndex(v2, vt2, vn2),
                    FaceIndex(v3, vt3, vn3)) => {

                    let ref vert1 = self.vertices[v1 as usize];
                    let ref norm1 = self.normals[vn1.unwrap() as usize];
                    let ref text1 = self.texture_coords[vt1.unwrap() as usize];

                    // TODO: finish the Interleaved transform. 
                    /*
                    data.v_vt_vn.push();
                    data.idx.push()
                    */

                }
                _ => {}
            }
        }
        data
    }
}

pub struct Interleaved {
    v_vt_vn: Vec<((f32, f32, f32, f32), (f32, f32, f32), (f32,f32,f32))>,
    idx: Vec<usize>
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test] fn cube_obj_has_12_faces() { // Triangulated model, 12/2 = 6 quads
        let Obj{ objects: cube_objects, .. } = Obj::create("assets/cube.obj");
        assert_eq!(cube_objects[0].faces().len(), 12);
    }

    #[test] fn cube_obj_has_8_verts() {
        let Obj{ objects: cube_objects, .. } = Obj::create("assets/cube.obj");
        assert_eq!(cube_objects[0].vertices().len(), 8);
    }

    #[test] fn cube_obj_has_1_object() {
        let Obj{ objects: cube_objects, .. }  = Obj::create("assets/cube.obj");
        assert_eq!(cube_objects.len(), 1);
    }

    #[test] fn parses_separate_objects() {
        let Obj{ objects: cube_objects, .. } = Obj::create("assets/four_blue_cubes.obj");
        assert_eq!(cube_objects.len(), 4);
    }


    #[test] fn its_a_test() {

    }
}