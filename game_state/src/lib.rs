// opinion here? reexport winit or import in other libs...?
pub use nalgebra;
pub use sdl2;
pub use sdl2::sys as sdl2_sys;

pub mod model;
pub mod state;
pub mod tree;

pub mod event;
pub mod input;
pub mod thing;
pub mod ui;
pub mod utils;

use std::sync::Arc;
use std::time::Duration;

use state::SceneGraph;
use std::sync::atomic::{AtomicUsize, Ordering};
use thing::CameraFacet;

static GLOBAL_IDENITY_CURSOR: AtomicUsize = AtomicUsize::new(0);

pub type Identity = u64;
pub fn create_next_identity() -> Identity {
    GLOBAL_IDENITY_CURSOR.fetch_add(1, Ordering::SeqCst) as Identity
}

pub trait Identifyable {
    fn identify(&self) -> Identity;
}

pub trait Renderer: Identifyable {
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
    fn present(&mut self, camera: &CameraFacet);
}

pub trait Behavior {
    fn update(delta_time: &Duration);
}
