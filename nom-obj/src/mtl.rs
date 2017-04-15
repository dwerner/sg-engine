/// http://paulbourke.net/dataformats/mtl/
/// 
use common::*;

use nom::{
    space,
    not_line_ending
};

use std::str;

named!(pub newmtl_line, delimited!( tag!("newmtl"), take_until!("\n"), end_of_line) );
named!(pub ambient_texture_line, delimited!( tag!("map_Ka"), take_until!("\n"), end_of_line) );
named!(pub diffuse_texture_line, delimited!( tag!("map_Kd"), take_until!("\n"), end_of_line) );
named!(pub specular_texture_line, delimited!( tag!("map_Ks"), take_until!("\n"), end_of_line) );

named!(pub ka_ambient_line< &[u8], (f32,f32,f32)>, delimited!(tag!("Ka"), float_triple, end_of_line));
named!(pub kd_diffuse_line< &[u8], (f32,f32,f32)>, delimited!(tag!("Kd"), float_triple, end_of_line));
named!(pub ks_specular_line< &[u8], (f32,f32,f32)>, delimited!(tag!("Ks"), float_triple, end_of_line));

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn can_parse_newmtl_line() {
        let (_,b) = newmtl_line("newmtl material/name\n".as_bytes()).unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " material/name");
    }

    #[test] fn can_parse_ambient_texture_line() {
        let (_,b) = ambient_texture_line("map_Ka sometexture.png\n".as_bytes()).unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " sometexture.png");
    }

    #[test] fn can_parse_diffuse_texture_line() {
        let (_,b) = diffuse_texture_line("map_Kd sometexture.png\n".as_bytes()).unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " sometexture.png");
    }

    #[test] fn can_parse_specular_texture_line() {
        let (_,b) = specular_texture_line("map_Ks sometexture.png\n".as_bytes()).unwrap();
        assert_eq!(str::from_utf8(b).unwrap(), " sometexture.png");
    }

    #[test] fn can_parse_ka_ambient_line() {
        let vline = "Ka 1.000 1.000 1.000  \r\n".as_bytes();
        let v = ka_ambient_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, (1.0, 1.0, 1.0));
    }
    #[test] fn can_parse_ka_diffuse_line() {
        let vline = "Kd 1.000 1.000 1.000  \r\n".as_bytes();
        let v = kd_diffuse_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, (1.0, 1.0, 1.0));
    }
    #[test] fn can_parse_ka_specular_line() {
        let vline = "Ks 1.000 1.000 1.000  \r\n".as_bytes();
        let v = ks_specular_line(vline);
        let (_,b) = v.unwrap();
        assert_eq!(b, (1.0, 1.0, 1.0));
    }

    const MTL_FILE: &'static str = "
newmtl Textured
d 0.9                    # some implementations use 'd'
Tr 0.1                   # others use 'Tr' (inverted: Tr = 1 - d)
Ka 1.000 1.000 1.000
Kd 1.000 1.000 1.000
Ks 0.000 0.000 0.000
d 1.0
illum 2
map_Ka lemur.tga           # the ambient texture map
map_Kd lemur.tga           # the diffuse texture map (most of the time, it will
                           # be the same as the ambient texture map)
map_Ks lemur.tga           # specular color texture map
map_Ns lemur_spec.tga      # specular highlight component
map_d lemur_alpha.tga      # the alpha texture map
map_bump lemur_bump.tga    # some implementations use 'map_bump' instead of 'bump' below
   bump lemur_bump.tga        # bump map (which by default uses luminance channel of the image)
   disp lemur_disp.tga        # displacement map
   decal lemur_stencil.tga    # stencil decal texture (defaults to 'matte' channel of the image)


    ";
}