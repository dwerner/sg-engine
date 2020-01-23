use std::collections::VecDeque;
use std::error::Error;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::{DescriptorSet, PersistentDescriptorSet};
use vulkano::descriptor::pipeline_layout::PipelineLayoutAbstract;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::attachment::AttachmentImage;
use vulkano::image::{
    ImageAccess, ImageLayout, ImageUsage, ImageViewAccess, ImmutableImage, MipmapsCount,
    SwapchainImage,
};
use vulkano::instance::debug::DebugCallback;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::raster::{
    CullMode, DepthBiasControl, FrontFace, PolygonMode, Rasterization,
};
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain;
use vulkano::swapchain::{Surface, SurfaceTransform, Swapchain};
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use game_state;
use game_state::input::screen::{ScreenPoint, ScreenRect};
use game_state::model::Model;
use game_state::state::DrawMode;
use game_state::state::SceneGraph;
use game_state::thing::CameraFacet;
use game_state::tree::BreadthFirstIterator;
use game_state::utils::fps;
use game_state::{Identifyable, Identity, Renderer};

use game_state::nalgebra::Matrix4;

pub mod vertex;
use self::vertex::Vertex;

pub mod vulkano_sdl2;

use vulkano_sdl2::WinPtr;

//TODO: compile these elsewhere, at build time?
// These shaders are a PITA, generated by build.rs, dependent on OUT_DIR... *barf
// More importantly, these are actually compiled SPIR-V, ignore the glsl file extension on them
/*
use vulkano::pipeline::shader::{
    GraphicsShaderType, ShaderInterfaceDef, ShaderInterfaceDefEntry, ShaderModule,
};
fn load_shader(device: Arc<Device>, path: PathBuf) -> Result<Arc<ShaderModule>, Box<Error>> {
    let mut f = File::open(&path)?;
    let mut v = vec![];
    f.read_to_end(&mut v)?;
    Ok(unsafe { ShaderModule::new(device, &v) }?)
}*/
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "../assets/shaders/vs.glsl"
    }
}
mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "../assets/shaders/fs.glsl"
    }
}

// ModelData is intented to encapsulate all Model+Material data that's specific to this
// Vulkano renderer - geometry, indices, materials
pub struct ModelData {
    pub model: Arc<Model>,
    pub vertices: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub indices: Arc<CpuAccessibleBuffer<[u16]>>,
    pub diffuse_map: Arc<CpuAccessibleBuffer<[[u8; 4]]>>,
    pub material_data: MaterialRenderData<vulkano::format::R8G8B8A8Srgb>,
}

// MaterialData holds the Vulkano handles to GPU images - `init` and `read` here alias the same
// image, however init is used to write the data, while read is used to read
// the descriptor_set is used to bind on a per-model basis during traversal of the scene graph
pub struct MaterialRenderData<F> {
    pub read: Arc<ImmutableImage<F>>,
    pub init: Arc<dyn ImageAccess>,
    pub descriptor_set: Arc<dyn DescriptorSet + Send + Sync>,
}

impl<F> MaterialRenderData<F> {
    pub fn new(
        read: Arc<ImmutableImage<F>>,
        init: Arc<dyn ImageAccess>,
        descriptor_set: Arc<dyn DescriptorSet + Send + Sync>,
    ) -> Self {
        MaterialRenderData {
            read,
            init,
            descriptor_set,
        }
    }
}

type ThisPipelineType = GraphicsPipeline<
    SingleBufferDefinition<vertex::Vertex>,
    Box<dyn PipelineLayoutAbstract + Send + Sync>,
    Arc<dyn vulkano::framebuffer::RenderPassAbstract + Send + Sync>,
>;

type ThisFramebufferType = Arc<dyn FramebufferAbstract + Send + Sync + 'static>;

pub struct VulkanoRenderer {
    id: Identity,
    instance: Arc<Instance>,
    surface: Arc<Surface<WinPtr>>,
    depth_buffer: Arc<dyn ImageViewAccess + Send + Sync>,
    events: Arc<Mutex<VecDeque<game_state::input::events::InputEvent>>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<WinPtr>>,
    images: Vec<Arc<SwapchainImage<WinPtr>>>,
    pipeline: Arc<ThisPipelineType>,
    framebuffers: Vec<ThisFramebufferType>,
    fps: fps::FPS,

    renderpass: Arc<dyn RenderPassAbstract + Send + Sync>,

    // TODO: camera
    uniform_buffer: Arc<CpuAccessibleBuffer<vs::ty::Data>>,

    render_layer_queue: VecDeque<Arc<SceneGraph>>,
    model_data: Vec<ModelData>,

    rect: ScreenRect,
    current_mouse_pos: ScreenPoint,

    // Enable vulkan debug layers? - need to install the vulkan sdk to get them
    #[allow(dead_code)]
    debug_callback: Option<vulkano::instance::debug::DebugCallback>,

    previous_frame_end: Box<dyn GpuFuture>,
    recreate_swapchain: bool,
    dynamic_state: DynamicState,
    fullscreen: bool,
    cursor_grabbed: bool,
    cursor_hidden: bool,
    cursor_wrapped: bool,
}

impl VulkanoRenderer {
    fn create_swapchain(
        surface: Arc<Surface<WinPtr>>,
        device: Arc<Device>,
        queue: Arc<Queue>,
        physical: PhysicalDevice,
    ) -> Result<(Arc<Swapchain<WinPtr>>, Vec<Arc<SwapchainImage<WinPtr>>>), String> {
        let caps = match surface.capabilities(physical.clone()) {
            Ok(caps) => caps,
            Err(err) => {
                return Err(format!(
                    "Unable to get capabilities from surface: {:?}",
                    err
                ))
            }
        };

        use vulkano::swapchain::PresentMode;

        let dimensions = caps.current_extent.unwrap_or([1280, 800]);
        //let present = caps.present_modes.iter().next().unwrap();
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        // note that some present modes block on vsync
        // TODO: this should be a user-configurable option
        // THOUGHTS: perhaps this could be better supported by putting the renderer on another thread
        // and then syncing with state once per update, but allowing rendering to happen
        // without blocking
        let present_mode = if caps.present_modes.immediate {
            Some(PresentMode::Immediate)
        } else if caps.present_modes.mailbox {
            Some(PresentMode::Mailbox)
        } else if caps.present_modes.relaxed {
            Some(PresentMode::Relaxed)
        } else if caps.present_modes.fifo {
            Some(PresentMode::Fifo)
        } else {
            None
        }
        .expect("No supported present mode found.");

        Ok(Swapchain::new(
            device,
            surface,
            caps.min_image_count,
            format,
            dimensions,
            1,
            caps.supported_usage_flags,
            &queue,
            SurfaceTransform::Identity,
            alpha,
            present_mode,
            true,
            None,
        )
        .expect("Failed to create swapchain."))
    }

    fn create_descriptor_set(
        device: Arc<Device>,
        uniform_buffer: Arc<CpuAccessibleBuffer<vs::ty::Data>>,
        queue: Arc<Queue>,
        pipeline: Arc<ThisPipelineType>,
        id: usize,
        texture: Arc<ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,
        width: u32,
        height: u32,
    ) -> Arc<dyn DescriptorSet + Send + Sync> {
        let sampler = vulkano::sampler::Sampler::new(
            device,
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::Filter::Linear,
            vulkano::sampler::MipmapMode::Nearest,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            vulkano::sampler::SamplerAddressMode::Repeat,
            0.0,
            1.0,
            0.0,
            0.0,
        )
        .unwrap();

        let ds = PersistentDescriptorSet::start(pipeline, 0) // intended to be bound at 0
            .add_sampled_image(texture, sampler)
            .expect("error loading texture")
            .add_buffer(uniform_buffer)
            .expect("error adding uniform buffer")
            .build()
            .unwrap();

        Arc::new(ds) as Arc<dyn DescriptorSet + Send + Sync>
    }

    fn create_framebuffers(
        width: u32,
        height: u32,
        renderpass: Arc<dyn RenderPassAbstract + Send + Sync + 'static>,
        images: Vec<Arc<SwapchainImage<WinPtr>>>,
        depth_buffer: Arc<dyn ImageViewAccess + Send + Sync + 'static>,
    ) -> Vec<ThisFramebufferType> {
        images
            .iter()
            .map(|image| {
                let dimensions = [width, height, 1];
                let fb = Framebuffer::with_dimensions(renderpass.clone(), dimensions)
                    .add(image.clone())
                    .unwrap()
                    .add(depth_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap();
                Arc::new(fb) as Arc<dyn FramebufferAbstract + Send + Sync + 'static>
            })
            .collect::<Vec<_>>()
    }

    pub fn new(win_ptr: WinPtr, draw_mode: DrawMode) -> Result<Self, Box<dyn Error>> {
        let instance = {
            let extensions = vulkano_sdl2::required_extensions(win_ptr).unwrap();
            let app_info = app_info_from_cargo_toml!();
            Instance::new(Some(&app_info), &extensions, None)
                .expect("Failed to create Vulkan instance. ")
        };

        let debug_callback = DebugCallback::errors_and_warnings(&instance, |msg| {
            println!("Debug callback: {:?}", msg.description);
        })
        .ok();

        let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
            .next()
            .expect("No device available.");

        let surface: Arc<Surface<WinPtr>> =
            vulkano_sdl2::build_vk_surface(win_ptr, instance.clone()).unwrap();

        let queue = physical
            .queue_families()
            .find(|q| q.supports_graphics() && surface.is_supported(q.clone()).unwrap_or(false))
            .expect("Couldn't find a graphical queue family.");

        let (device, mut queues) = {
            let device_ext = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                ..vulkano::device::DeviceExtensions::none()
            };

            Device::new(
                physical,
                physical.supported_features(),
                &device_ext,
                [(queue, 0.5)].iter().cloned(),
            )
            .expect("Failed to create device.")
        };

        let queue = queues.next().unwrap();

        let (swapchain, images) =
            Self::create_swapchain(surface.clone(), device.clone(), queue.clone(), physical)?;

        // TODO: as part of asset_loader, we should be loading all the shaders we expect to use in a scene
        let vs = vs::Shader::load(device.clone()).expect("failed to create vs shader module");
        let fs = fs::Shader::load(device.clone()).expect("failed to create fs shader module");

        // ----------------------------------
        // Uniform buffer
        // TODO: extract to the notion of a camera

        let proj = Matrix4::new_perspective(
            {
                let d = ImageAccess::dimensions(&images[0]);
                d.width() as f32 / d.height() as f32
            }, // aspect
            ::std::f32::consts::FRAC_PI_2,
            0.01,
            100.0, // depth used for culling!
        );

        let uniform_buffer = CpuAccessibleBuffer::<vs::ty::Data>::from_data(
            device.clone(),
            vulkano::buffer::BufferUsage::all(),
            vs::ty::Data { proj: proj.into() },
        )
        .expect("failed to create uniform buffer");

        // ----------------------------------

        let img_usage = ImageUsage {
            transient_attachment: true,
            input_attachment: true,
            ..ImageUsage::none()
        };
        let depth_buffer = AttachmentImage::with_usage(
            device.clone(),
            SwapchainImage::dimensions(&images[0]),
            vulkano::format::D16Unorm,
            img_usage,
        )
        .unwrap();

        #[allow(dead_code)]
        let renderpass = single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),//ImageAccess::format(&images[0]),
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: Store,
                    format: vulkano::image::ImageAccess::format(&depth_buffer),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        )
        .unwrap();

        let renderpass = Arc::new(renderpass); //as Arc<RenderPassAbstract + Send + Sync>;
        let depth_buffer = Arc::new(depth_buffer); //
        let dimensions = ImageAccess::dimensions(&images[0]);
        let framebuffers = Self::create_framebuffers(
            dimensions.width(),
            dimensions.height(),
            renderpass.clone(),
            images.clone(),
            depth_buffer.clone(),
        );

        // -----------------------------------------------
        // Rendermodes, fill, lines, points
        let mut raster = Rasterization::default();
        raster.cull_mode = CullMode::Back;
        raster.polygon_mode = match draw_mode {
            DrawMode::Colored => PolygonMode::Fill,
            DrawMode::Points => PolygonMode::Point,
            DrawMode::Wireframe => PolygonMode::Line,
        };
        raster.depth_clamp = true;
        raster.front_face = FrontFace::Clockwise;
        raster.line_width = Some(2.0);
        raster.depth_bias = DepthBiasControl::Dynamic;
        // -------------------------------------------------

        let p = GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .polygon_mode_fill()
            .depth_clamp(true)
            .cull_mode_front()
            .front_face_counter_clockwise()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .blend_alpha_blending()
            .render_pass(
                Subpass::from(
                    renderpass.clone() as Arc<dyn RenderPassAbstract + Send + Sync>,
                    0,
                )
                .unwrap(),
            )
            .build(device.clone())?;

        let pipeline = Arc::new(p);

        // finish up by grabbing some initialization values for position and size
        let (x, y) = (0.0, 0.0);
        let (width, height) = (800.0, 600.0);

        // TODO: get actual mouse position... or does it matter at this point when we get it in the
        // event loop instead

        let previous_frame_end = Box::new(now(device.clone())) as Box<dyn GpuFuture>;
        let instance = instance.clone();
        let events = Arc::new(Mutex::new(VecDeque::new()));

        Ok(VulkanoRenderer {
            id: game_state::create_next_identity(),
            instance,
            surface,
            events,
            device,
            queue,
            swapchain,
            images,
            pipeline,
            depth_buffer,
            framebuffers,
            uniform_buffer,
            debug_callback,
            previous_frame_end,
            renderpass: renderpass as Arc<dyn RenderPassAbstract + Send + Sync>,
            recreate_swapchain: false, // flag indicating to rebuild the swapchain on the next frame
            model_data: Vec::new(),
            render_layer_queue: VecDeque::new(),
            fps: fps::FPS::new(),
            current_mouse_pos: ScreenPoint::new(0, 0),
            rect: ScreenRect::new(x as i32, y as i32, width as i32, height as i32),

            fullscreen: false,
            cursor_grabbed: false,
            cursor_hidden: false,
            cursor_wrapped: false,
            // TODO: should DynamicState be reset when the swapchain is rebuilt as well?
            dynamic_state: DynamicState {
                line_width: None,
                viewports: Some(vec![vulkano::pipeline::viewport::Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [dimensions.width() as f32, dimensions.height() as f32],
                    depth_range: 0.0..1.0,
                }]),
                ..DynamicState::none()
            },
        })
    }

    #[inline]
    fn get_mouse_pos(&self) -> &ScreenPoint {
        &self.current_mouse_pos
    }

    #[inline]
    fn set_mouse_pos(&mut self, pos: ScreenPoint) {
        self.current_mouse_pos = pos;
    }

    #[allow(dead_code)]
    #[inline]
    fn get_rect(&self) -> &ScreenRect {
        &self.rect
    }

    #[inline]
    fn set_rect(&mut self, new_rect: ScreenRect) {
        // TODO: let the renderer know to change things up because we were resized?
        self.flag_recreate_swapchain();

        // TODO: determine a delta here?
        self.rect = new_rect;
    }

    pub fn upload_model(&mut self, model: Arc<game_state::model::Model>) {
        {
            // save model+material in VulkanoRenderer buffer cache
            let mesh = &model.mesh;
            let vertices: Vec<Vertex> = mesh.vertices.iter().map(|x| Vertex::from(*x)).collect();

            let pixel_buffer = {
                let image = model.material.diffuse_map.to_rgba();
                let image_data = image.into_raw().clone();

                let image_data_chunks = image_data.chunks(4).map(|c| [c[0], c[1], c[2], c[3]]);

                // TODO: staging buffer instead
                vulkano::buffer::cpu_access::CpuAccessibleBuffer::<[[u8; 4]]>::from_iter(
                    self.device.clone(),
                    BufferUsage::all(),
                    image_data_chunks,
                )
                .expect("failed to create buffer")
            };

            // TODO: per-model textures are 2048x2048, perhaps this could depend on the image instead?

            let (texture, texture_init) = ImmutableImage::uninitialized(
                self.device.clone(),
                vulkano::image::Dimensions::Dim2d {
                    width: 2048,
                    height: 2048,
                },
                vulkano::format::R8G8B8A8Srgb,
                MipmapsCount::One,
                ImageUsage {
                    transfer_source: true, // for blits
                    transfer_destination: true,
                    sampled: true,
                    ..ImageUsage::none()
                },
                ImageLayout::ShaderReadOnlyOptimal,
                Some(self.queue.family()),
            )
            .unwrap();

            let texture_init = Arc::new(texture_init);

            let pipeline_set = Self::create_descriptor_set(
                self.device.clone(),
                self.uniform_buffer.clone(),
                self.queue.clone(),
                self.pipeline.clone(),
                0, // we intend this descriptor_set to fit in with the pipeline at set 0
                texture.clone(),
                2048,
                2048,
            );

            let item = ModelData {
                model: model.clone(),
                vertices: CpuAccessibleBuffer::from_iter(
                    self.device.clone(),
                    BufferUsage::all(),
                    vertices.iter().cloned(),
                )
                .expect("Unable to create buffer"),
                indices: CpuAccessibleBuffer::from_iter(
                    self.device.clone(),
                    BufferUsage::all(),
                    mesh.indices.iter().cloned(),
                )
                .expect("Unable to create buffer"),
                diffuse_map: pixel_buffer,
                material_data: MaterialRenderData::new(
                    texture.clone(),
                    texture_init.clone(),
                    pipeline_set.clone(),
                ),
            };

            // upload to GPU memory
            let cmd_buffer_build = AutoCommandBufferBuilder::primary_one_time_submit(
                self.device.clone(),
                self.queue.family(),
            )
            .unwrap(); // catch oom error here

            let cmd_buffer = cmd_buffer_build
                .copy_buffer_to_image(item.diffuse_map.clone(), texture_init)
                .expect("unable to upload texture")
                .build()
                .expect("unable to build command buffer");

            let prev = mem::replace(
                &mut self.previous_frame_end,
                Box::new(now(self.device.clone())) as Box<dyn GpuFuture>,
            );

            let execute = match prev.then_execute(self.queue.clone(), cmd_buffer) {
                Ok(execute) => execute,
                Err(e) => {
                    println!("VulkanoRenderer::upload_model() frame {} - unable to execute command buffer {:?}", self.fps.count(), e);
                    return;
                }
            };

            let after_future = execute.then_signal_fence_and_flush();

            match after_future {
                Ok(future) => {
                    self.model_data.push(item);
                    self.previous_frame_end = Box::new(future) as Box<_>;
                }
                Err(e) => {
                    println!("Error ending frame {:?}", e);
                    self.previous_frame_end =
                        Box::new(vulkano::sync::now(self.device.clone())) as Box<_>;
                }
            }
        }
    }

    fn flag_recreate_swapchain(&mut self) {
        self.recreate_swapchain = true;
    }

    fn render(&mut self, camera: &CameraFacet) {
        self.previous_frame_end.cleanup_finished();

        if self.recreate_swapchain {
            //println!("recreating swapchain with dimensions {:?}", size);
            use vulkano::swapchain::SwapchainCreationError;

            let physical = vulkano::instance::PhysicalDevice::enumerate(&self.instance)
                .next()
                .expect("no device availble");

            let dims = self
                .surface
                .capabilities(physical)
                .expect("failed to get surface capabilities")
                .current_extent
                .unwrap_or([1024, 768]);

            match self.swapchain.recreate_with_dimension(dims) {
                Ok((new_swapchain, new_images)) => {
                    self.swapchain = new_swapchain;
                    self.images = new_images;

                    self.depth_buffer = AttachmentImage::transient(
                        self.device.clone(),
                        dims,
                        vulkano::format::D16Unorm,
                    )
                    .unwrap();

                    let dimensions = ImageAccess::dimensions(&self.images[0]);
                    self.framebuffers = Self::create_framebuffers(
                        dimensions.width(),
                        dimensions.height(),
                        self.renderpass.clone(),
                        self.images.clone(),
                        self.depth_buffer.clone(),
                    );

                    self.dynamic_state = DynamicState {
                        line_width: None,
                        viewports: Some(vec![vulkano::pipeline::viewport::Viewport {
                            origin: [0.0, 0.0],
                            dimensions: [dims[0] as f32, dims[1] as f32],
                            depth_range: 0.0..1.0,
                        }]),
                        ..DynamicState::none()
                    };

                    self.recreate_swapchain = false;
                }
                Err(SwapchainCreationError::UnsupportedDimensions) => {
                    println!("Unsupported dimensions! {:?}", dims);
                }
                Err(e) => panic!("{:?}", e),
            }
        }

        // HACKY Note the use of 300 micros as a magic number for acquiring a swapchain image
        let (image_num, acquire_future) = match swapchain::acquire_next_image(
            self.swapchain.clone(),
            Some(Duration::from_micros(300)),
        ) {
            Ok((num, future)) => (num, future),
            Err(vulkano::swapchain::AcquireError::OutOfDate) => {
                self.flag_recreate_swapchain();
                return;
            }
            Err(vulkano::swapchain::AcquireError::Timeout) => {
                println!("swapchain::acquire_next_image() Timeout!");
                return;
            }
            Err(e) => panic!("{:?}", e),
        };

        let mut cmd_buffer_build = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )
        .unwrap(); // catch oom error here

        cmd_buffer_build = cmd_buffer_build
            .begin_render_pass(
                self.framebuffers[image_num].clone(),
                false,
                vec![
                    vulkano::format::ClearValue::from([0.0, 0.0, 0.0, 1.0]),
                    vulkano::format::ClearValue::Depth(1.0),
                ],
            )
            .expect("unable to begin renderpass");

        let view = camera.view;

        let scale = Matrix4::new_scaling(1.0);
        let viewscale = view * scale;

        // TODO: WIP implement a notion of a camera
        // TODO: do we want to do this every frame?
        let proj_mat = Matrix4::new_perspective(
            {
                let d = ImageAccess::dimensions(&self.images[0]);
                d.width() as f32 / d.height() as f32
            },
            ::std::f32::consts::FRAC_PI_2,
            0.01,
            100.0, // depth used for culling!
        );

        // modify the data in the uniform buffer for this renderer == our camera
        match self.uniform_buffer.write() {
            Ok(mut uniform) => uniform.proj = proj_mat.into(),
            Err(err) => {} // println!("Error writing to uniform buffer {:?}", err),
        }

        while let Some(next_layer) = self.render_layer_queue.pop_front() {
            // TODO: refactor this to use asset lookups
            // TODO: refactor this to use WorldEntity collection -> SceneGraph Rc types
            // TODO: asset lookups should store DescriptorSets with associated textures

            let iterator = BreadthFirstIterator::new(next_layer.root.clone());
            for (_node_id, rc) in iterator {
                let node = &mut rc.borrow_mut();

                // TODO: implement a per model -instance- matrix in the graph itself?
                let model = self.model_data[node.data as usize].model.clone();

                let model_mat = model.model_mat;

                // TODO: update the world matrices from the parent * child's local matrix
                // eg. flag dirty a node, which means all children must be updated
                // actually save the data in each node
                let transform_mat = node
                    .parent()
                    .map(|parent| {
                        let parent_model_id = parent.borrow().data;
                        let parent_model = &self.model_data[parent_model_id as usize].model;
                        parent_model.world_mat * model_mat
                    })
                    .unwrap_or(model_mat);

                // Push constants are leveraged here to send per-model
                // matrices into the shaders
                let push_constants = vs::ty::PushConstants {
                    model_mat: (viewscale * transform_mat).into(),
                };

                let mdl = &self.model_data[node.data as usize];

                cmd_buffer_build = cmd_buffer_build
                    .draw_indexed(
                        self.pipeline.clone(),
                        &self.dynamic_state,
                        mdl.vertices.clone(),
                        mdl.indices.clone(),
                        mdl.material_data.descriptor_set.clone(),
                        push_constants, // or () - both leak on win32...
                    )
                    .expect("Unable to add command");
            }
        }

        let cmd_buffer = cmd_buffer_build
            .end_render_pass()
            .expect("unable to end renderpass ")
            .build()
            .unwrap();

        let prev = mem::replace(
            &mut self.previous_frame_end,
            Box::new(now(self.device.clone())) as Box<dyn GpuFuture>,
        );

        let after_future = match prev
            .join(acquire_future)
            .then_execute(self.queue.clone(), cmd_buffer)
        {
            Ok(executed) => executed
                .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                .then_signal_fence_and_flush(),
            Err(e) => {
                self.fps.update();
                println!(
                    "VulkanoRenderer::render() frame {} - unable to execute command buffer, {:?}",
                    self.fps.count(),
                    e
                );
                return;
            }
        };

        match after_future {
            Ok(future) => {
                self.previous_frame_end = Box::new(future) as Box<_>;
            }
            Err(vulkano::sync::FlushError::OutOfDate) => {
                println!(
                    "swapchain is out of date, flagging recreate_swapchain=true for next frame"
                );

                self.flag_recreate_swapchain();
                self.previous_frame_end =
                    Box::new(vulkano::sync::now(self.device.clone())) as Box<_>;
            }
            Err(e) => {
                println!("Error ending frame {:?}", e);
                self.previous_frame_end =
                    Box::new(vulkano::sync::now(self.device.clone())) as Box<_>;
            }
        }

        self.fps.update();
    }

    #[allow(dead_code)]
    fn fps(&self) -> f32 {
        self.fps.get()
    }
}

impl Identifyable for VulkanoRenderer {
    fn identify(&self) -> Identity {
        self.id
    }
}

impl Renderer for VulkanoRenderer {
    fn load(&mut self) {}

    fn unload(&mut self) {
        self.model_data.clear();
    }

    fn queue_render_layer(&mut self, layer: Arc<SceneGraph>) {
        self.render_layer_queue.push_back(layer);
    }

    fn present(&mut self, camera: &CameraFacet) {
        self.render(camera);
    }
}

impl Drop for VulkanoRenderer {
    fn drop(&mut self) {
        println!("VulkanRenderer drop");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn rando_test_flatten_vec_of_options() {
        let vals = vec![
            None,
            None,
            Some(1),
            None,
            Some(2),
            Some(3),
            None,
            None,
            None,
            Some(4),
        ];
        let flat = vals
            .iter()
            .enumerate()
            .filter(|&(_, x)| x.is_some())
            .map(|(_, x)| x.unwrap())
            .collect::<Vec<u32>>();
        assert_eq!(flat, vec![1, 2, 3, 4]);
    }
}
