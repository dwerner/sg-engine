use std::{env,};

extern crate vulkano_shaders;
fn main() {
    println!("{:?}", env::var("OUT_DIR").unwrap() );
    vulkano_shaders::build_glsl_shaders([
        ("../assets/shaders/vs.glsl", vulkano_shaders::ShaderType::Vertex),
        ("../assets/shaders/fs.glsl", vulkano_shaders::ShaderType::Fragment),
    ].iter().cloned());
}
