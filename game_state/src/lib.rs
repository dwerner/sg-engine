pub mod state;
pub mod tree;
pub mod model;

pub mod ui;
pub mod input;
pub mod event;
pub mod utils;
pub mod thing;

extern crate cgmath;
extern crate nom_obj;
extern crate image;

pub extern crate time;
pub extern crate winit;

// #[macro_use]
// extern crate serde_derive;
// extern crate bincode;
// extern crate serde_json;

use cgmath::Matrix4;

use model::Mesh;
use std::sync::Arc;

use state::SceneGraph;

use tree::RcNode;

use input::InputSource;

// Represents the public interface for mods
// traits for implementing behavior of state objects should exist here
// but the impls for those traits can be in mods

pub type Identity = u64; // really?

use std::sync::atomic::{ AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
static GLOBAL_IDENITY_CURSOR: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn create_next_identity() -> Identity {
    GLOBAL_IDENITY_CURSOR.fetch_add(1, Ordering::SeqCst) as Identity
}

pub trait Identifyable {
    fn identify(&self) -> Identity;
}

use state::State;
use thing::CameraFacet;

pub trait Renderer : Identifyable + InputSource {

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
    fn present(&mut self, camera: &CameraFacet<f32>);

}

pub trait Behavior {
    fn update(delta_time: &time::Duration);
}

