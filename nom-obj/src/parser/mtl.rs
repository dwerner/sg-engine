/// http://paulbourke.net/dataformats/mtl/
/// 
use parser::common::*;

use nom::{
    space,
    not_line_ending
};

use std::str;

#[derive(PartialEq, Debug)]
pub struct FaceIndex(u32, Option<u32>, Option<u32>);

#[derive(PartialEq, Debug)]
pub enum MtlLine<'a> {
    Comment(&'a str),
    NewMtl(&'a str),
    AmbientMap(&'a str),
    DiffuseMap(&'a str),
    SpecularMap(&'a str),
    BumpMap(&'a str),

    AmbientColor(f32, f32, f32),
    DiffuseColor(f32, f32, f32),
    SpecularColor(f32, f32, f32),
    KeColor(f32,f32,f32), // unknown, but blender writes it

    TransmissionFilter(f32,f32,f32),

    OpticalDensity(f32),
    SpecularExponent(u32),
    TransparencyD(f32),
    TransparencyTr(f32),
    IlluminationModel(u32),
    Sharpness(u32)
}

def_string_line!(newmtl_line, "newmtl", MtlLine, NewMtl);
def_string_line!(ambient_texture_line, "map_Ka", MtlLine, AmbientMap);
def_string_line!(diffuse_texture_line, "map_Kd", MtlLine, DiffuseMap);
def_string_line!(specular_texture_line, "map_Ks", MtlLine, SpecularMap);
def_string_line!(bump_texture_line, "map_bump", MtlLine, BumpMap);

named!(pub ka_ambient_line< &[u8], MtlLine >, map!(
    delimited!(tag!("Ka"), float_triple, end_of_line), |(r,g,b)| MtlLine::AmbientColor(r,g,b)
));

named!(pub transmission_filter_line< &[u8], MtlLine >, map!(
    delimited!(tag!("Tf"), float_triple, end_of_line), |(r,g,b)| MtlLine::TransmissionFilter(r,g,b)
));

named!(pub kd_diffuse_line< &[u8], MtlLine >, map!(
    delimited!(tag!("Kd"), float_triple, end_of_line), |(r,g,b)| MtlLine::DiffuseColor(r,g,b)
));

named!(pub ks_specular_line< &[u8], MtlLine >, map!(
    delimited!(tag!("Ks"), float_triple, end_of_line), |(r,g,b)| MtlLine::SpecularColor(r,g,b)
));

named!(pub ke_line< &[u8], MtlLine >, map!(
    delimited!(tag!("Ke"), float_triple, end_of_line), |(r,g,b)| MtlLine::KeColor(r,g,b)
));

named!(pub transparency_line_d< &[u8], MtlLine >, map!(
    sp!(delimited!(tag!("d"), float, end_of_line)), |t| MtlLine::TransparencyD(t)
));

named!(pub transparency_line_tr< &[u8], MtlLine >, map!(
    sp!(delimited!(tag!("Tr"), float, end_of_line)), |t| MtlLine::TransparencyTr(t)
));

named!(pub optical_density_line< &[u8], MtlLine >, map!(
    sp!(delimited!(tag!("Ni"), float, end_of_line)), |t| MtlLine::OpticalDensity(t)
));

named!(pub illum_line< &[u8], MtlLine >, map!(
    sp!(delimited!(tag!("illum"), uint, end_of_line)), |t| MtlLine::IlluminationModel(t)
));

named!(pub sharpness_line< &[u8], MtlLine >, map!(
    sp!(delimited!(tag!("sharpness"), uint, end_of_line)), |t| MtlLine::Sharpness(t)
));

named!(pub specular_exponent_line< &[u8], MtlLine >, map!(
    sp!(delimited!(tag!("Ns"), uint, end_of_line)), |t| MtlLine::SpecularExponent(t)
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn can_parse_newmtl_line() {
        let (_,b) = newmtl_line("newmtl material/name\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::NewMtl("material/name"));
    }

    #[test] fn can_parse_ambient_texture_line() {
        let (_,b) = ambient_texture_line("map_Ka sometexture.png\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::AmbientMap("sometexture.png"));
    }

    #[test] fn can_parse_diffuse_texture_line() {
        let (_,b) = diffuse_texture_line("map_Kd sometexture.png\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::DiffuseMap("sometexture.png"));
    }

    #[test] fn can_parse_specular_texture_line() {
        let (_,b) = specular_texture_line("map_Ks sometexture.png\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::SpecularMap("sometexture.png"));
    }

    #[test] fn can_parse_transparency_d_line() {
        let (_,b) = transparency_line_d("d 0.5\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::TransparencyD(0.5));
    }

    #[test] fn can_parse_transparency_tr_line() {
        let (_,b) = transparency_line_tr("Tr 0.5\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::TransparencyTr(0.5));
    }

    #[test] fn can_parse_illumination_model_line() {
        let (_,b) = illum_line("illum 2\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::IlluminationModel(2));
    }

    #[test] fn can_parse_specular_exponent_line() {
        let (_,b) = specular_exponent_line("Ns 2\n".as_bytes()).unwrap();
        assert_eq!(b, MtlLine::SpecularExponent(2));
    }

    #[test] fn can_parse_ka_ambient_line() {
        let vline = "Ka 1.000 1.000 1.000  \r\n".as_bytes();
        let v = ka_ambient_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, MtlLine::AmbientColor(1.0, 1.0, 1.0));
    }
    #[test] fn can_parse_ka_diffuse_line() {
        let vline = "Kd 1.000 1.000 1.000  \r\n".as_bytes();
        let v = kd_diffuse_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, MtlLine::DiffuseColor(1.0, 1.0, 1.0));
    }
    #[test] fn can_parse_ka_specular_line() {
        let vline = "Ks 1.000 1.000 1.000  \r\n".as_bytes();
        let v = ks_specular_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, MtlLine::SpecularColor(1.0, 1.0, 1.0));
    }

    const MTL_FILE: &'static str = "
# Blender MTL File: 'None'
# Material Count: 1

newmtl Material.002
Ns 96.078431
Ka 1.000000 1.000000 1.000000
Kd 0.000000 0.003667 0.640000
Ks 0.500000 0.500000 0.500000
Ke 0.000000 0.000000 0.000000
Ni 1.000000
d 0.600000
illum 2
";
}