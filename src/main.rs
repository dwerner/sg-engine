/*
 *	 Idea: arbitrary model support through observables (a framework for simulation state sync?)
 *
 *   - Synchronization model
 *
 *   http://gafferongames.com/networked-physics/snapshot-compression/
 *
 *   https://github.com/rygorous/gaffer_net/blob/master/main.cpp
 *
 *   Target bandwidth range vs Actual bandwidth use
 *
 * 	 ????
 *   - separate sync mechanisms for separate properties
 *   - prioritization of sync for different properties
 *   - adaptive sync methodology
 *   - express changing values as rate of change for interp
 *   - trans Simuation migration of objects
 *   - support simulation-level persistence (binary file, and maybe redis?)
 *   - property bounding (limit range, define quantization)
 *   - custom property serialization traits (e.g. quaternion's 'smallest three')
 *   - delta compression - send only what has changed
 *   - arbitrary precision types (like varint)
 *   - desync handling
 *   
 *   Knobs:
 *	 - snapshot send rate (per second)
 *	 - packet size
 *	 - interpolation between snapshots in buffer
 *	 - size of snapshot buffer
 *	 - extrapolation of velocities, linear and angular
 *	 - protocol (tcp/udp) - udp send/ack_send
 *	 - data compression (none, zlib, compress)
 *
 *	 Detections:
 *	 - snapshot length in bytes
 *	 - bandwidth
 *	 - latency
 *	 - packet loss
 *
 *	 Deterministic Lock-step
 *	 Snapshots and Interpolation (send all state)
 *	 State synchronization
 *
 *	 p2p vs client/server
 *
 *	 Gameworld = [ x x x x x x ] ==> [ x x x x x y ] === [ 6x, x->y ]
 *
 *	 more scrap:
 *	 		- sync priority (level of detail for syncs)
 *	 			- near points of interest (high-low) etc
 *
 *
 *	 'Object model'
 *
 *	 SimSync
 *	  \-> Schedule
 *	 	  \-> Object
 *	 		  \-> Object
 *	 			  \-> ...
 */

/*
 * SyncUpdate - the main 
 */

extern crate rustc_serialize;
extern crate bincode;
extern crate capnp;
extern crate time;

#[macro_use]
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;
extern crate glsl_to_spirv;
extern crate vulkano_shaders;

use vulkano_win::VkSurfaceBuild;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer;
use vulkano::command_buffer::DynamicState;
use vulkano::command_buffer::PrimaryCommandBufferBuilder;
use vulkano::command_buffer::Submission;
use vulkano::descriptor::pipeline_layout::EmptyPipeline;
use vulkano::device::Device;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::instance::Instance;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineParams;
use vulkano::pipeline::blend::Blend;
use vulkano::pipeline::depth_stencil::DepthStencil;
use vulkano::pipeline::input_assembly::InputAssembly;
use vulkano::pipeline::multisample::Multisample;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::viewport::ViewportsState;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::viewport::Scissor;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;

use std::sync::Arc;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode/*, decode*/};

use rustc_serialize::{json /*, Encodable, Decodable*/};

extern crate engine;
use engine::renderer;

//use std::fmt::Debug;

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct SyncUpdate<M> {
	sequence: u32, 
	data: M
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
enum Anything {
	Nothing,
	Everything(u8),
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct Datum {
	v: u8,
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct Datum2(u8);

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct BoolDat(bool);

fn main() {

	let pwd = std::env::current_dir();
	println!("{:?}", pwd);

	let mut vec = Vec::with_capacity(256);
	for x in 0u8..255 {
		vec.push(BoolDat( x > 124));
	}

	let update = SyncUpdate { sequence: 42, data: vec };
	let update_str = json::encode(&update).unwrap().to_string();
	let update_bin = encode(&update, SizeLimit::Infinite).unwrap();

	//println!("update_str {}", update_str);
	println!("update_str len: {}", update_str.len());
	println!("update_bin len: {}", update_bin.len());


	// Vulkan 
	let instance = {
		let extensions = vulkano_win::required_extensions();
		Instance::new(None, &extensions, None).expect("Failed to create Vulkan instance.")
	};

	let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
		.next().expect("No device available.");

	let window = winit::WindowBuilder::new().build_vk_surface(&instance).unwrap();

	let queue = physical.queue_families().find(|q| {
		q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
	}).expect("Couldn't find a graphical queue family.");

	let (device, mut queues) = {
		let device_ext = vulkano::device::DeviceExtensions {
			khr_swapchain: true,
			.. vulkano::device::DeviceExtensions::none()
		};

		Device::new(&physical, physical.supported_features(), &device_ext,
			[(queue, 0.5)].iter().cloned()
		).expect("Failed to create device.")
	};

	let queue = queues.next().unwrap();

	let (swapchain, images) = {
		let caps = window.surface().get_capabilities(&physical).expect("Failed to get surface capabilities");
		let dimensions = caps.current_extent.unwrap_or([1280, 800]);
		let present = caps.present_modes.iter().next().unwrap();
		let alpha = caps.supported_composite_alpha.iter().next().unwrap();
		let format = caps.supported_formats[0].0;
		Swapchain::new(&device, &window.surface(), 2, format, dimensions, 1,
			&caps.supported_usage_flags, &queue, SurfaceTransform::Identity, alpha,
			present, true, None).expect("Failed to create swapchain.")
	};

	let vertex_buffer = {
		#[derive(Debug, Clone)]
		struct Vertex { position: [f32;2] }
		impl_vertex!(Vertex, position);

		CpuAccessibleBuffer::from_iter(
			&device,
			&BufferUsage::all(),
			Some(queue.family()),
				[
					Vertex { position: [-0.5, -0.25] }, 
					Vertex { position: [0.0, 0.5] }, 
					Vertex { position: [0.25, -0.1] },

					Vertex { position: [0.5, 0.25] }, 
					Vertex { position: [0.0, -0.5] }, 
					Vertex { position: [-0.25, 0.1] },
				].iter().cloned()
		).expect("Failed to create vertex buffer")
	};

	mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/assets/shaders/triangle_vs.glsl") }}
	let vs = vs::Shader::load(&device).expect("failed to create vs shader module");
	mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/assets/shaders/triangle_fs.glsl") }}
	let fs = fs::Shader::load(&device).expect("failed to create fs shader module");

	let geo_shader_src = r"
#version 330 core
layout (points) in;
layout (line_strip, max_vertices = 2) out;
void main() {    
	gl_Position = gl_in[0].gl_Position + vec4(-0.1, 0.0, 0.0, 0.0); 
	EmitVertex();
	gl_Position = gl_in[0].gl_Position + vec4(0.1, 0.0, 0.0, 0.0);
	EmitVertex();
	EndPrimitive();
}";

	//let geometry_shader = glsl_to_spirv::compile(geo_shader_src, glsl_to_spirv::ShaderType::Geometry).unwrap();

	mod render_pass {
		use vulkano::format::Format;
		single_pass_renderpass!{
			attachments: {
				color: {
					load:Clear,
					store:Store,
					format:Format,
				}
			},
			pass: {
				color: [color],
				depth_stencil: {}
			}
		}
	}

	let render_pass = render_pass::CustomRenderPass::new(&device, &render_pass::Formats {
		color: (images[0].format(), 1)
	}).unwrap();

	let pipeline = GraphicsPipeline::new(&device, GraphicsPipelineParams {
		vertex_input: SingleBufferDefinition::new(),
		vertex_shader: vs.main_entry_point(),
		input_assembly: vulkano::pipeline::input_assembly::InputAssembly {
			topology: vulkano::pipeline::input_assembly::PrimitiveTopology::TriangleStrip,
			primitive_restart_enable: false,
		},
		tessellation: None,
		geometry_shader: None, //&geometry_shader,
		viewport: ViewportsState::Fixed {
			data: vec![(
				Viewport {
					origin: [0.0, 0.0],
					depth_range: 0.0 .. 1.0,
					dimensions: [images[0].dimensions()[0] as f32,
											 images[0].dimensions()[1] as f32],
				},
				Scissor::irrelevant()
			)],
		},
		raster: Default::default(),
		multisample: Multisample::disabled(),
		fragment_shader: fs.main_entry_point(),
		depth_stencil: DepthStencil::disabled(),
		blend: Blend::pass_through(),
		layout: &EmptyPipeline::new(&device).unwrap(),
		render_pass: Subpass::from(&render_pass, 0).unwrap(),
	}).unwrap();

	let framebuffers = images.iter().map(|image| {
		let dimensions = [image.dimensions()[0], image.dimensions()[1], 1];
		Framebuffer::new(&render_pass, dimensions, render_pass::AList {
			color: image
		}).unwrap()
	}).collect::<Vec<_>>();

	let mut submissions:Vec<Arc<Submission>> = Vec::new();


	window.window().set_title("Something good...");

	let mut fps = renderer::utils::fps::FPS::new();
	let mut frame = 0;

	'running: loop {
		frame +=1;
		fps.update();
		let frame_rate = fps.get();

		if frame_rate > 60.0 {
			std::thread::sleep(std::time::Duration::from_millis(16));
		}

		if frame % 100 == 0 {
			println!("FPS: {}", frame_rate);
		}

		submissions.retain(|s| s.destroying_would_block() );
		let image_num = swapchain.acquire_next_image(std::time::Duration::new(1,0)).unwrap();
		let command_buffer = PrimaryCommandBufferBuilder::new(&device, queue.family())
			.draw_inline(&render_pass, &framebuffers[image_num], render_pass::ClearValues {
				color: [1.0, 0.0, 0.5, 1.0]
			})
			.draw(&pipeline, &vertex_buffer, &DynamicState::none(), (), &())
			.draw_end()
		.build();

		submissions.push(command_buffer::submit(&command_buffer, &queue).unwrap());
		swapchain.present(&queue, image_num).unwrap();

		// Make use of winit
		for ev in window.window().poll_events() {
			match ev {
				winit::Event::Closed => {
					println!("Window closed.");
					return;
				},
				_ => ()
			}
		}
	}
}
