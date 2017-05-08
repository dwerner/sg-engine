pub mod state;
pub mod tree;
pub mod model;

pub mod ui;
pub mod input;
pub mod event;

extern crate cgmath;
extern crate nom_obj;
extern crate image;
extern crate time;

use cgmath::Matrix4;

use model::Mesh;
use std::sync::Arc;

use state::SceneGraph;

use tree::RcNode;

// Represents the public interface for mods
// traits for implementing behavior of state objects should exist here
// but the impls for those traits can be in mods

pub type Identity = u64;
use std::sync::atomic::{ AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
static GLOBAL_IDENITY_CURSOR: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn create_next_identity() -> Identity {
    GLOBAL_IDENITY_CURSOR.fetch_add(1, Ordering::SeqCst) as Identity
}

pub trait Identifyable {
    fn identify(&self) -> Identity;
}

pub trait Renderer : Identifyable {

    /// load()
    /// provide a hook for a mod to notify the renderer that it is about to be used
    fn load(&mut self);

    /// unload()
    /// This is called by a mod to notify the renderer to be done with any state.
    fn unload(&mut self);

    /// queue_render_layer()
    /// Set the renderer up with a queue of SceneGraphs
    fn queue_render_layer(&mut self, layer: Arc<SceneGraph>);

    /// present()
    /// Actually render the image, compositing render layers in the order they were queued
    fn present(&mut self);

    // get_input_events() <- renderer should be tracking input events
    fn get_input_events(&mut self) -> Vec<input::events::InputEvent>;


    // Window handle
    fn set_title(&mut self, title: &str) {}

}

pub trait Renderable : Identifyable {
    fn get_mesh(&self) -> &Mesh;

    fn get_diffuse_map(&self) -> &image::DynamicImage;
    fn get_world_matrix(&self) -> &Matrix4<f32>;
    fn set_world_matrix(&mut self, mat: Matrix4<f32>);

    fn get_model_matrix(&self) -> &Matrix4<f32>;
    fn set_model_matrix(&mut self, mat: Matrix4<f32>);

    // TODO fn get_graph_node(&self) -> tree::RcNode<_>;
}

pub struct PhysicalComponent {
    pub mass: f32,
    pub linear_velocity: model::Vector,
    pub angular_velocity: model::Vector,
    pub position: model::Vector,
}

pub enum Shape {
    Box { width: f32, height: f32, depth: f32 },
    Cone { radius: f32, height: f32 },
    Cylinder { radius: f32, height: f32 },
    Sphere { radius: f32 },
}

pub trait PhysicalBody<T> {
    fn set_position(&mut self, model::Vector);
    fn get_position(&self) -> &model::Vector;

    fn set_mass(&mut self, m: f32);
    fn get_mass(&self) -> &model::Vector;

    fn set_linear_velocity(&mut self, model::Vector);
    fn get_linear_velocity(&self) -> &model::Vector;

    fn set_angular_velocity(&mut self, model::Vector);
    fn get_angular_velocity(&self) -> &model::Vector;

    fn update(delta_time: &time::Duration);

    fn get_body(&self) -> &Shape;
    fn set_body(&mut self, shape: Shape);
}

pub trait Behavior {
    fn update(delta_time: &time::Duration);
}

pub struct SceneGraphNode;
pub struct PhysicalGraphNode;

// Entity component system, Optional components?
// WorldEntity
// - renderable : strong reference to node in renderable tree
// - body : strong reference to node in physical object tree
// ? behavior?
pub struct WorldEntity {
    id: Identity,
    renderable: Option< RcNode<SceneGraphNode> >,
    body: Option< RcNode<PhysicalGraphNode> >,
}
