#[macro_use]
extern crate nom;
use nom::{
    IResult,
    digit,
    eol
};

use std::str;
use std::str::FromStr;

pub struct ObjParser {
}

impl ObjParser {
    pub fn new() -> Self {
        ObjParser{}
    }
}

named!(whitespace, eat_separator!(&b" \t"[..]));
macro_rules! sp (
   ($i:expr, $($args:tt)*) => (
     {
       sep!($i, whitespace, $($args)*)
     }
   )
 );

named!(slashes, eat_separator!(&b"/"[..]));
macro_rules! slash_sep (
   ($i:expr, $($args:tt)*) => (
     {
       sep!($i, slashes, $($args)*)
     }
   )
 );


named!(end_of_line, alt!(eof!() | eol));

named!(unsigned_float <f32>, map_res!(
  map_res!(
    recognize!(
      alt!(
        delimited!(digit, tag!("."), opt!(digit)) |
        delimited!(opt!(digit), tag!("."), digit)
      )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(float <f32>, map!(
   pair!(
     opt!(alt!(tag!("+") | tag!("-"))),
     unsigned_float
   ),
   |(sign, value): (Option<&[u8]>, f32)| {
     sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f32) } else { None }).unwrap_or(1f32) * value
   }
));

named!(uint <u32>, map_res!(map_res!( recognize!( digit ), str::from_utf8 ), FromStr::from_str));


named!( comment, delimited!(
    tag!("#"),
    take_until!("\n"),
    end_of_line
));
named!( object_line, delimited!( tag!("o"), take_until!("\n"), end_of_line) );
named!( mtllib_line, delimited!(tag!("mtllib"), take_until!("\n"), end_of_line) );
named!( usemtl_line, delimited!(tag!("usemtl"), take_until!("\n"), end_of_line) );
named!( s_line, delimited!(tag!("s"), take_until!("\n"), end_of_line) );

named!( float_triple< &[u8], (f32,f32,f32) >, sp!(tuple!(float, float, float)));
named!( float_pair< &[u8], (f32,f32) >,  sp!(tuple!(float, float)));

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

named!( face_triple< &[u8], (u32, Option<u32>, Option<u32>) >,
    sp!(slash_sep!(
        tuple!(uint, opt!(uint), opt!(uint))
    ))
);


named!( face_line< &[u8], ((u32, Option<u32>, Option<u32>), (u32, Option<u32>, Option<u32>), (u32, Option<u32>, Option<u32>)) >, sp!(
    delimited!(
        tag!("f"),
        tuple!(face_triple, face_triple, face_triple),
        end_of_line
    )
));


pub enum ObjLine {
    Comment(&'a str),
    ObjectName(&'a str),
    MaterialLibrary(&'a str),
    SLine(&'a str),
    Vertex(f32, f32, f32),
    Normal(f32, f32, f32),
    Face(u32,u32,u32),
    TextureCoord(f32,f32),
}

// TODO: tie parsers together into a single one that can chunk through a file


#[cfg(test)]
mod tests {

    use super::*;

    #[test] fn can_parse_face_triple() {
        let ff = face_triple(" 1/11/4".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b, (1, Some(11), Some(4)) );
    }


    #[test] fn can_parse_face_line() {
        let ff = face_line("f 1/11/4  1/11/4 1/11/4  \n".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b,
            (
                (1, Some(11), Some(4)),
                (1, Some(11), Some(4)),
                (1, Some(11), Some(4))
            )
        );
    }

    #[test] fn can_parse_signed_floats() {
        let something = float("-0.00005".as_bytes());
        assert_eq!(something, IResult::Done(&b""[..], -0.00005));
    }

    #[test]
    fn can_parse_float_pair() {
        let ff = float_pair("     -1.000001 7742.9 ".as_bytes());
        let (_,b) = ff.unwrap();
        assert_eq!(b, (-1.000001, 7742.9));
    }

    #[test]
    fn can_parse_float_triple() {
        let fff = float_triple("    0.95  -1.000001 42.9 ".as_bytes());
        let (_,b) = fff.unwrap();
        assert_eq!(b, (0.95, -1.000001, 42.9));
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

    #[test] fn can_parse_comments() {
        let cmt = comment("# a comment exists here \n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " a comment exists here ");
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