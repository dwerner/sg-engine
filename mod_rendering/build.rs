use std::{env,};

extern crate vulkano_shaders;
fn main() {
    println!("OUT_DIR {:?}", env::var("OUT_DIR").unwrap() );
    println!("current_dir {:?}", env::current_dir().unwrap() );
    vulkano_shaders::build_glsl_shaders([
        ("../assets/shaders/vs.glsl", vulkano_shaders::ShaderType::Vertex),
        ("../assets/shaders/fs.glsl", vulkano_shaders::ShaderType::Fragment),
    ].iter().cloned());
}
