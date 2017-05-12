use nom::{
    digit,
    eol
};

use std::str;
use std::str::FromStr;

named!(pub whitespace, eat_separator!(&b" \t"[..]));

#[macro_export]
macro_rules! sp (
   ($i:expr, $($args:tt)*) => (
     {
       sep!($i, whitespace, $($args)*)
     }
   )
 );

named!(pub slashes, eat_separator!(&b"/"[..]));

#[macro_export]
macro_rules! slash_sep (
   ($i:expr, $($args:tt)*) => (
     {
       sep!($i, slashes, $($args)*)
     }
   )
 );

/// Meta macro, define parser for lines starting with $i, map to enum $tt type $ty
/// Converts to &str and trims whitespace from line
#[macro_export]
macro_rules! def_string_line (
   ($id:ident, $i:expr, $tt:tt, $ty:ident) => (
       named!( $id< &[u8], $tt >, map!(
           delimited!(tag!($i), take_until!("\n"), end_of_line),
           |s| $tt :: $ty(str::from_utf8(s).unwrap().trim().to_string())
       ));
   )
);

named!(pub comment, delimited!(
    tag!("#"),
    take_until!("\n"),
    alt!( eof!() | eol )
));

named!(pub end_of_line, alt!(
    eof!()
    |
    eol
    |
    comment  // handle end of line comments - these are not kept
));

named!(pub unsigned_float <f32>, map_res!(
  map_res!(
    recognize!(
      alt!(
        delimited!(digit, complete!(tag!(".")), opt!(complete!(digit)))
        |
        delimited!(opt!(digit), complete!(tag!(".")), digit)
        |
        digit
      )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(pub float <f32>, map!(
   pair!(
     opt!(alt!(tag!("+") | tag!("-"))),
     unsigned_float
   ),
   |(sign, value): (Option<&[u8]>, f32)| {
     sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f32) } else { None }).unwrap_or(1f32) * value
   }
));

named!(pub uint <u32>, map_res!(map_res!( recognize!( digit ), str::from_utf8 ), FromStr::from_str));


named!(pub float_triple_opt_4th< &[u8], (f32,f32,f32,Option<f32>)>, sp!(
    tuple!( float, float, float, opt!(float) )
));

named!(pub float_pair_opt_3rd< &[u8], (f32,f32,Option<f32>) >, sp!(
    tuple!(float, float, opt!(float))
));

named!(pub float_triple< &[u8], (f32,f32,f32) >, sp!(tuple!(float, float, float)));
named!(pub float_pair< &[u8], (f32,f32) >,  sp!(tuple!(float, float)));

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test] fn can_parse_comments() {
        let cmt = comment("# a comment exists here \n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " a comment exists here ");
    }

    #[test] fn can_parse_comments_2() {
        let cmt = comment("# Blender v2.78 (sub 0) OBJ File: \'untitled.blend\'\n".as_bytes());
        let (_,b) = cmt.unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " Blender v2.78 (sub 0) OBJ File: \'untitled.blend\'");
    }
}
