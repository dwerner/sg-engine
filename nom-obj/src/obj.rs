/// http://paulbourke.net/dataformats/obj/
///
use common::*;

use nom::{
   space
};

use std::str;

named!( object_line, delimited!( tag!("o"), take_until!("\n"), end_of_line) );
named!( mtllib_line, delimited!(tag!("mtllib"), take_until!("\n"), end_of_line) );
named!( usemtl_line, delimited!(tag!("usemtl"), take_until!("\n"), end_of_line) );
named!( s_line, delimited!(tag!("s"), take_until!("\n"), end_of_line) );

named!( vertex_line< &[u8], (f32,f32,f32) >, sp!(
    delimited!(
        tag!("v"),
        float_triple,
        end_of_line
    )
));

named!( normal_line< &[u8], (f32,f32,f32) >, sp!(
    delimited!(
        tag!("vn"),
        float_triple,
        end_of_line
    )
));

named!( texcoord_line< &[u8], (f32,f32) >,   sp!(
    delimited!(
        tag!("vt"),
        float_pair,
        end_of_line
    )
));

named!( vertex_param_line< &[u8], (f32,f32,f32) >, sp!(
    delimited!(
        tag!("vp"),
        float_triple,
        end_of_line
    )
));

named!( face_triple< &[u8], (u32, Option<u32>, Option<u32>) >, tuple!(
        uint,
        delimited!( tag!("/"), opt!(uint), tag!("/") ),
        opt!(uint)
    )
);

named!( face_pair< &[u8], (u32, Option<u32>) >, separated_pair!(
        uint,
        tag!("/"),
        opt!(uint)
    )
);

named!( face_line< &[u8], (
    (u32, Option<u32>, Option<u32>),
    (u32, Option<u32>, Option<u32>),
    (u32, Option<u32>, Option<u32>)) >, delimited!(
        sp!(tag!("f")),
        alt!(
            sp!(tuple!(uint, uint, uint)) => {|(u1,u2,u3)| ((u1, None, None), (u2, None, None), (u3, None, None))}
            |
            sp!(tuple!(face_pair, face_pair, face_pair)) => {|((u1, v1),(u2,v2),(u3,v3))|
                ((u1, v1, None), (u2, v2, None), (u3, v3, None))
            }
            |
            sp!(tuple!(face_triple, face_triple, face_triple))
        ),
        end_of_line
    )
);


pub enum ObjLine<'a> {
    Comment(&'a str),
    ObjectName(&'a str),
    MaterialLibrary(&'a str),
    SLine(&'a str),
    Vertex(f32, f32, f32, f32), // x, y, z, then w defaults to 1.0
    VertexParam(f32, f32, f32),
    Normal(f32, f32, f32),
    Face(u32, Option<u32>, Option<u32>),
    TextureUVW(f32, f32, f32), // u,v, then w defaults to 0.0
}

pub struct ObjParser {
}

impl ObjParser {
    pub fn new() -> Self {
        ObjParser{}
    }
}

// TODO: tie parsers together into a single one that can chunk through a file


#[cfg(test)]
mod tests {

    use super::*;


    #[test] fn can_ignore_comment_at_eol() {
        let ff = face_line("f 1/11/4 1/3/4 1/11/4  #this is an important face \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        (
            (1, Some(11), Some(4)),
            (1, Some(3), Some(4)),
            (1, Some(11), Some(4))
        )
        );
    }


    #[test] fn can_parse_face_triple() {
        named!(sp_face< (u32, Option<u32>, Option<u32>) >, sp!(face_triple));
        let ff = face_triple("1/11/4".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b, (1, Some(11), Some(4)) );
    }

    #[test] fn can_parse_face_line_1() {
        let ff = face_line("f 1/11/4 1/3/4 1/11/4  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        (
            (1, Some(11), Some(4)),
            (1, Some(3), Some(4)),
            (1, Some(11), Some(4))
        )
        );
    }

    #[test] fn can_parse_face_line_2() {
        //
        let ff = face_line("f 1/3 2/62 4/3\n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        (
            (1, Some(3), None),
            (2, Some(62), None),
            (4, Some(3), None),
        )
        );
    }

    #[test] fn can_parse_face_line_3() {
        let ff = face_line("f 1//4 1//4 1//11  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        (
            (1, None, Some(4)),
            (1, None, Some(4)),
            (1, None, Some(11))
        )
        );
    }

    #[test] fn can_parse_face_line_4() {
        let ff = face_line("f 42 1 11  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        (
            (42, None, None),
            (1, None, None),
            (11, None, None)
        )
        );
    }

    #[test] fn can_parse_face_line_5() {
        let ff = face_line("f 42/ 1/ 11/  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        (
            (42, None, None),
            (1, None, None),
            (11, None, None)
        )
        );
    }

    #[test] fn can_parse_face_line_6() {
        let ff = face_line("f 42// 1// 11// \t \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
        (
            (42, None, None),
            (1, None, None),
            (11, None, None)
        )
        );
    }

    #[test] fn can_parse_texcoord_line() {
        let vline = "vt -1.000000 -1.000000 \r\n".as_bytes();
        let v = texcoord_line(vline);
        let (_a,b) = v.unwrap();
        assert_eq!(b, (-1.0, -1.0));
    }

    #[test] fn can_parse_normal_line() {
        let vline = "vn -1.000000 -1.000000 1.000000  \r\n".as_bytes();
        let v = normal_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, (-1.0, -1.0, 1.0));
    }

    #[test]
    #[should_panic]
    fn can_parse_vertex_line_2() {
        let vline = "vZZ -1.000000 -1.000000 1.000000 \r\n".as_bytes();
        let v = vertex_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, (-1.0, -1.0, 1.0));
    }

    #[test]
    fn can_parse_vertex_parameter_line() {
        let vline = "vp -1.000000 -1.000000 1.000000 \r\n".as_bytes();
        let v = vertex_param_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, (-1.0, -1.0, 1.0));
    }

    #[test]
    fn can_parse_vertex_line() {
        let vline = "v -1.000000 -1.000000 1.000000 \r\n".as_bytes();
        let v = vertex_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, (-1.0, -1.0, 1.0));
    }

    #[test] fn can_parse_object_line() {
        let cmt = object_line("o someobject.999asdf.7 \n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " someobject.999asdf.7 ");
    }

    #[test] fn can_parse_mtllib_line() {
        let cmt = mtllib_line("mtllib somelib \n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " somelib ");
    }

    #[test] fn can_parse_usemtl_line() {
        let cmt = usemtl_line("usemtl SomeMaterial\n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " SomeMaterial");
    }

    #[test] fn can_parse_s_line() {
        let cmt = s_line("s off\n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " off");
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
