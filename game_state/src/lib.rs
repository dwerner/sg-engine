pub mod state;
pub mod tree;
pub mod model;

pub mod ui;
pub mod input;
pub mod event;


extern crate cgmath;
extern crate obj;

use cgmath::Matrix4;

use model::Mesh;
use std::sync::Arc;

use state::SceneGraph;

// Represents the public interface for mods
// traits for implementing behavior of state objects should exist here
// but the impls for those traits can be in mods

pub trait Renderer {

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

    // force these to be implemented soon
    // get_input_events() <- renderer should be tracking input events
    fn get_input_events(&self) -> Vec<input::events::InputEvent> {
        Vec::new()
    }

    // Window handle
    fn set_title(&mut self, title: &str) {}
}

pub trait Identifyable {
    fn identify(&self) -> u64;
}

pub trait Renderable : Identifyable {
    fn get_mesh(&self) -> &Mesh;

    fn get_world_matrix(&self) -> &Matrix4<f32>;
    fn set_world_matrix(&mut self, mat: Matrix4<f32>);

    fn get_model_matrix(&self) -> &Matrix4<f32>;
    fn set_model_matrix(&mut self, mat: Matrix4<f32>);
}
