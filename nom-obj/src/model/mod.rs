use parser::obj::{
    ObjParser,
    ObjLine,
    FaceIndex,
};

use parser::mtl::{
    MtlParser,
    MtlLine
};

// use parser::mtl::{ MtlLine, MtlParser };

pub struct Obj {
    pub filename: String,
    pub comments: Vec<ObjLine>,
    pub objects: Vec<ObjObject>,
}
impl Obj {
    pub fn create(filename: &str) -> Self {
        let mut obj = Obj{
            filename: filename.to_string(),
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
                ObjLine::TextureUVW(..) => object.texture_coords.push(line),
                _ => {}
            }
        }
        obj.objects.push(object);
        obj
    }

    pub fn get_mtl(&self) -> ObjMaterial {

        use std::path::{
            Path,
            PathBuf
        };
        let source = Path::new(&self.filename);
        let parent = source.parent().unwrap().to_str().unwrap();

        let mut obj_material = ObjMaterial {
            diffuse_map: "".to_string()
        };
        // todo : support more than one object here
        let maybe_material = self.objects[0].material();
        let material = maybe_material.unwrap();

        let whole_path: PathBuf = Path::join(Path::new(parent), material);
        let material = whole_path.to_owned().to_str().unwrap().to_string();

        let parser = MtlParser::create(&material);
        for line in parser {
            match line {
                MtlLine::DiffuseMap(filename) => {
                    let whole_path: PathBuf = Path::join(Path::new(parent), filename.clone());
                    obj_material.diffuse_map = whole_path.to_owned().to_str().unwrap().to_string();
                    break; // TODO: handle more than a single material
                }
                _ => {}
            }
        }
        obj_material
    }
}

#[derive(Debug)]
pub struct ObjObject {
    pub name: Option<String>,
    pub material: Option<String>,
    vertices: Vec<ObjLine>,
    normals: Vec<ObjLine>,
    texture_coords: Vec<ObjLine>,
    vertex_params: Vec<ObjLine>,
    faces: Vec<ObjLine>,
}

#[derive(Debug)]
pub struct ObjMaterial {
    pub diffuse_map: String
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

    pub fn material(&self) -> Option<String> {
        match &self.material {
            &Some(ref value) => Some(value.clone()),
            &None => None
        }
    }

    pub fn name(&self) -> &Option<String> { &self.name }
    pub fn faces(&self) -> &Vec<ObjLine> { &self.faces }

    #[inline]
    fn get_v_tuple(&self, face_index: &FaceIndex) -> (f32,f32,f32,f32) {
        let &FaceIndex(v, _, _) = face_index;
        match &self.vertices[(v as usize) - 1] {
            &ObjLine::Vertex(x,y,z,w) => (x,y,z,w.unwrap_or(1.0)),
            _ => panic!("not a vertex")
        }
    }


    #[inline]
    fn get_vt_tuple(&self, face_index: &FaceIndex) -> (f32,f32,f32) {
        let &FaceIndex(_, vt, _) = face_index;
        if vt.is_none() { (0.0,0.0,0.0) } else {
            match &self.texture_coords[(vt.unwrap() as usize) - 1] {
                &ObjLine::TextureUVW(u, v, w) => (u, v, w.unwrap_or(0.0)),
                _ => panic!("not a vertex")
            }
        }
    }

    #[inline]
    fn get_vn_tuple(&self, face_index: &FaceIndex) -> (f32,f32,f32) {
        let &FaceIndex(_, _, vn) = face_index;
        if vn.is_none() { (0.0,0.0,0.0) } else {
            match &self.normals[(vn.unwrap() as usize) - 1] {
                &ObjLine::Normal(x, y, z) => (x, y, z),
                _ => panic!("not a vertex")
            }
        }
    }

    #[inline]
    fn interleave_tuples(&self, id: &FaceIndex) -> (
        (f32, f32, f32, f32),
        (f32, f32, f32),
        (f32,f32,f32)
    ) {
        let vert = self.get_v_tuple(id);
        let text = self.get_vt_tuple(id);
        let norm = self.get_vn_tuple(id);
        (vert, text, norm)
    }

    pub fn interleaved(&self) -> Interleaved {
        use std::collections::HashMap;

        let mut vertex_map = HashMap::new();

        let mut data = Interleaved{ v_vt_vn: Vec::new(), idx:Vec::new() };

        for i in 0usize..self.faces.len() {
            match &self.faces[i] {
                &ObjLine::Face(ref id1, ref id2, ref id3) => {

                    let next_idx = (id1.0 as usize) - 1;
                    data.idx.push(next_idx);
                    vertex_map.entry(next_idx).or_insert(self.interleave_tuples(id1));

                    let next_idx = (id2.0 as usize) - 1;
                    data.idx.push(next_idx);
                    vertex_map.entry(next_idx).or_insert(self.interleave_tuples(id2));

                    let next_idx = (id3.0 as usize) - 1;
                    data.idx.push(next_idx);
                    vertex_map.entry(next_idx).or_insert(self.interleave_tuples(id3));

                }
                _ => { panic!("Found something other than a ObjLine::Face in object.faces") }
            }
        }
        for i in 0usize..vertex_map.len() {
            data.v_vt_vn.push(vertex_map.remove(&i).unwrap());
        }
        data
    }
}

pub struct Interleaved {
    pub v_vt_vn: Vec<((f32, f32, f32, f32), (f32, f32, f32), (f32, f32, f32))>,
    pub idx: Vec<usize>
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test] fn cube_format_interleaved() {
        let obj = Obj::create("assets/cube.obj");
        let interleaved = obj.objects[0].interleaved();
        println!("{:?}", obj.objects[0].faces());
        assert_eq!(obj.objects[0].faces().len(), 12);
        assert_eq!(interleaved.v_vt_vn.len(), 8);

        let ObjMaterial { diffuse_map } = obj.get_mtl();
        // TODO : fix this to always use unix paths
        assert_eq!(diffuse_map, "assets\\diffuse_map.png");
        //TODO assert_eq!(diffuse_map, "assets/diffuse_map.png");
    }

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