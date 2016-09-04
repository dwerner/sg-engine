extern crate vulkano_shaders;

fn main() {
	let pwd = std::env::current_dir();
	println!("{:?}", pwd);
	vulkano_shaders::build_glsl_shaders([
		("shaders/src/bin/triangle_vs.glsl", vulkano_shaders::ShaderType::Vertex),
		("shaders/src/bin/triangle_fs.glsl", vulkano_shaders::ShaderType::Fragment),
	].iter().cloned());
}
