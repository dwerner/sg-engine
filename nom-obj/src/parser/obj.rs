/// http://paulbourke.net/dataformats/obj/
///
use parser::common::*;

use std::str;

#[derive(PartialEq, Debug)]
pub struct FaceIndex(pub u32, pub Option<u32>, pub Option<u32>);

#[derive(PartialEq, Debug)]
pub enum ObjLine {
    Comment(String),
    ObjectName(String),
    GroupName(String),
    MtlLib(String),
    UseMtl(String),
    SmoothShading(String),
    Vertex(f32, f32, f32, Option<f32>), // x, y, z, then w defaults to 1.0
    VertexParam(f32, f32, f32),
    Normal(f32, f32, f32),
    Face(FaceIndex, FaceIndex, FaceIndex),
    TextureUVW(f32, f32, Option<f32>), // u,v, then w defaults to 0.0
}

def_string_line!(object_line, "o", ObjLine, ObjectName);
def_string_line!(group_line, "g", ObjLine, GroupName);
def_string_line!(mtllib_line, "mtllib", ObjLine, MtlLib);
def_string_line!(usemtl_line, "usemtl", ObjLine, UseMtl);
def_string_line!(s_line, "s", ObjLine, SmoothShading);

named!( vertex_line< &[u8], ObjLine >, map!(
    sp!( delimited!( tag!("v"), float_triple_opt_4th, end_of_line )),
    |(x,y,z,w)| ObjLine::Vertex(x,y,z,w)
));

named!( normal_line< &[u8], ObjLine >, map!(
    sp!( delimited!( tag!("vn"), float_triple, end_of_line )),
    |(x,y,z)| ObjLine::Normal(x,y,z)
));

named!( texcoord_line< &[u8], ObjLine >, map!(
    sp!( delimited!( tag!("vt"), float_pair_opt_3rd, end_of_line )),
    |(u,v,w)| ObjLine::TextureUVW(u,v,w)
));

named!( vertex_param_line< &[u8], ObjLine >, map!(
    sp!(delimited!( tag!("vp"), float_triple, end_of_line )),
    |(x,y,z)| ObjLine::VertexParam(x,y,z)
));

named!( face_triple< &[u8], FaceIndex >, map!(
    tuple!(
        uint,
        delimited!( tag!("/"), opt!(uint), tag!("/") ),
        opt!(uint)
    ),
    |(v, vt, vn)| FaceIndex(v, vt, vn)
));

named!( face_pair< &[u8], FaceIndex >, map!(
    separated_pair!(
        uint,
        tag!("/"),
        opt!(uint)
    ),
    |(v,vt)| FaceIndex(v, vt, None)
));

named!( face_line< &[u8], ObjLine >, delimited!(
        sp!(tag!("f")),
        alt!(
            sp!(tuple!(uint, uint, uint)) => {|(u1,u2,u3)| ObjLine::Face(
                FaceIndex(u1, None, None),
                FaceIndex(u2, None, None),
                FaceIndex(u3, None, None)
                )
            }
            |
            sp!(tuple!(face_pair, face_pair, face_pair)) => {|(a,b,c)| ObjLine::Face(a,b,c)}
            |
            sp!(tuple!(face_triple, face_triple, face_triple)) =>  {|(a,b,c)| ObjLine::Face(a,b,c)}
        ),
        end_of_line
    )
);

named!(comment_line< ObjLine >, map!(
    sp!(comment),
    |s| ObjLine::Comment(str::from_utf8(s).unwrap().trim().to_string())
));

named!(parse_obj_line< ObjLine >, alt!(
    vertex_line
    | normal_line
    | vertex_param_line
    | texcoord_line
    | face_line
    | object_line
    | group_line
    | mtllib_line
    | usemtl_line
    | s_line
    | comment_line
));


use std::fs::File;
use std::io::BufReader;
pub struct ObjParser {
    filename: String,
    reader: BufReader<File>,
}

impl ObjParser {
    pub fn create(filename: &str) -> Self {
        let reader = BufReader::new(File::open(filename).expect("Unable to open file"));
        ObjParser{
            filename: filename.to_string(),
            reader: reader,
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}

impl Iterator for ObjParser {
    type Item = ObjLine;

    fn next(&mut self) -> Option<Self::Item> {
        use nom::IResult;
        use std::io::BufRead;
        let mut line = String::new();
        let read_result = self.reader.read_line(&mut line);
        match read_result {
            Ok(len) => if len > 0 {
                let result = parse_obj_line(line.as_bytes());
                match result {
                    IResult::Done(_, o) => { Some(o) },
                    IResult::Error(_e) => { None },
                    IResult::Incomplete(_) => {
                        self.next()
                    },
                }
            } else {
                None
            },
            Err(_o) => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test] fn parser_can_read_from_file() {
        let parser = ObjParser::create("assets/cube.obj");
        let parsed_lines = parser.collect::<Vec<_>>();
        assert_eq!(parsed_lines.len(), 51);
    }

    #[test] fn can_parse_any_line() {
        let result = parse_obj_line("f 1/11/4 1/3/4 1/11/4  #this is an important face \n".as_bytes());
        let (_, line) = result.unwrap();
        assert_eq!(line,
            ObjLine::Face(
                FaceIndex(1, Some(11), Some(4)),
                FaceIndex(1, Some(3), Some(4)),
                FaceIndex(1, Some(11), Some(4))
            )
        );
    }

    #[test] fn can_ignore_comment_at_eol() {
        let ff = face_line("f 1/11/4 1/3/4 1/11/4  #this is an important face \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
            ObjLine::Face(
                FaceIndex(1, Some(11), Some(4)),
                FaceIndex(1, Some(3), Some(4)),
                FaceIndex(1, Some(11), Some(4))
            )
        );
    }


    #[test] fn can_parse_face_triple() {
        named!(sp_face< FaceIndex >, sp!(face_triple));
        let ff = face_triple("1/11/4".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b, FaceIndex(1, Some(11), Some(4)) );
    }

    #[test] fn can_parse_face_line_1() {
        let ff = face_line("f 1/11/4 1/3/4 1/11/4  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        ObjLine::Face(
            FaceIndex(1, Some(11), Some(4)),
            FaceIndex(1, Some(3), Some(4)),
            FaceIndex(1, Some(11), Some(4))
        )
        );
    }

    #[test] fn can_parse_face_line_2() {
        //
        let ff = face_line("f 1/3 2/62 4/3\n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        ObjLine::Face(
            FaceIndex(1, Some(3), None),
            FaceIndex(2, Some(62), None),
            FaceIndex(4, Some(3), None),
        )
        );
    }

    #[test] fn can_parse_face_line_3() {
        let ff = face_line("f 1//4 1//4 1//11  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        ObjLine::Face(
            FaceIndex(1, None, Some(4)),
            FaceIndex(1, None, Some(4)),
            FaceIndex(1, None, Some(11))
        )
        );
    }

    #[test] fn can_parse_face_line_4() {
        let ff = face_line("f 42 1 11  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        ObjLine::Face(
            FaceIndex(42, None, None),
            FaceIndex(1, None, None),
            FaceIndex(11, None, None)
        )
        );
    }

    #[test] fn can_parse_face_line_5() {
        let ff = face_line("f 42/ 1/ 11/  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        ObjLine::Face(
            FaceIndex(42, None, None),
            FaceIndex(1, None, None),
            FaceIndex(11, None, None)
        )
        );
    }

    #[test] fn can_parse_face_line_6() {
        let ff = face_line("f 42// 1// 11// \t \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        ObjLine::Face(
            FaceIndex(42, None, None),
            FaceIndex(1, None, None),
            FaceIndex(11, None, None)
        )
        );
    }

    #[test] fn can_parse_texcoord_line() {
        let vline = "vt -1.000000 -1.000000 \r\n".as_bytes();
        let v = texcoord_line(vline);
        let (_a,b) = v.unwrap();
        assert_eq!(b, ObjLine::TextureUVW(-1.0, -1.0, None));
    }

    #[test] fn can_parse_normal_line() {
        let vline = "vn -1.000000 -1.000000 1.000000  \r\n".as_bytes();
        let v = normal_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, ObjLine::Normal(-1.0, -1.0, 1.0));
    }

    #[test]
    #[should_panic]
    fn invalid_vertex_line_fails() {
        let vline = "vZZ -1.000000 -1.000000 1.000000 \r\n".as_bytes();
        let v = vertex_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, ObjLine::Vertex(-1.0, -1.0, 1.0, None));
    }

    #[test]
    fn can_parse_vertex_parameter_line() {
        let vline = "vp -1.000000 -1.000000 1.000000 \r\n".as_bytes();
        let v = vertex_param_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, ObjLine::VertexParam(-1.0, -1.0, 1.0));
    }

    #[test]
    fn can_parse_vertex_line_with_optional_w_value() {
        let vline = "v -1.000000 -1.000000 1.000000 42.000\r\n".as_bytes();
        let v = vertex_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, ObjLine::Vertex(-1.0, -1.0, 1.0, Some(42.0)));
    }

    #[test]
    fn can_parse_vertex_line() {
        let vline = "v -1.000000 -1.000000 1.000000 \r\n".as_bytes();
        let v = vertex_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, ObjLine::Vertex(-1.0, -1.0, 1.0, None));
    }

    #[test] fn can_parse_object_line() {
        let cmt = object_line("o someobject.999asdf.7 \n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(b, ObjLine::ObjectName("someobject.999asdf.7".to_string()));
    }

    #[test] fn can_parse_mtllib_line() {
        let cmt = mtllib_line("mtllib somelib \n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(b, ObjLine::MtlLib("somelib".to_string()));
    }

    #[test] fn can_parse_usemtl_line() {
        let cmt = usemtl_line("usemtl SomeMaterial\n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(b, ObjLine::UseMtl("SomeMaterial".to_string()));
    }

    #[test] fn can_parse_s_line() {
        let cmt = s_line("s off\n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(b, ObjLine::SmoothShading("off".to_string()));
    }

    const CUBE_MODEL: &'static str = "
# Blender v3.78 (sub 0) OBJ File: 'untitled.blend'
# www.blender.org
mtllib cube.mtl
o Cube_Cube.001
v -1.000000 -1.000000 1.000000
v -1.000000 1.000000 1.000000
v -1.000000 -1.000000 -1.000000
v -1.000000 1.000000 -1.000000
v 1.000000 -1.000000 1.000000
v 1.000000 1.000000 1.000000
v 1.000000 -1.000000 -1.000000
v 1.000000 1.000000 -1.000000
vt 0.0000 0.0000
vt 1.0000 0.0000
vt 1.0000 1.0000
vt 0.0000 0.0000
vt 1.0000 0.0000
vt 1.0000 1.0000
vt 0.0000 0.0000
vt 1.0000 0.0000
vt 1.0000 1.0000
vt 0.0000 0.0000
vt 1.0000 0.0000
vt 1.0000 1.0000
vt 0.0000 0.0000
vt 1.0000 0.0000
vt 1.0000 1.0000
vt 1.0000 0.0000
vt 1.0000 0.0000
vt 1.0000 0.0000
vt 1.0000 1.0000
vn -1.0000 0.0000 0.0000
vn 0.0000 0.0000 -1.0000
vn 1.0000 0.0000 0.0000
vn 0.0000 0.0000 1.0000
vn 0.0000 -1.0000 0.0000
vn 0.0000 1.0000 0.0000
usemtl None
s off
f 2/1/1 3/2/1 1/3/1
f 4/4/2 7/5/2 3/6/2
f 8/7/3 5/8/3 7/9/3
f 6/10/4 1/11/4 5/12/4
f 7/13/5 1/11/5 3/6/5
f 4/4/6 6/14/6 8/15/6
f 2/1/1 4/16/1 3/6/1
f 4/4/2 8/17/2 7/9/2
f 8/7/3 6/14/3 5/12/3
f 6/10/4 2/18/4 1/3/4
f 7/13/5 5/8/5 1/3/5
f 4/4/6 2/18/6 6/19/6
";

}
