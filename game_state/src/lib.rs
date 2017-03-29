pub mod state;
pub mod tree;
pub mod model;

pub mod ui;
pub mod input;
pub mod event;


extern crate cgmath;

use cgmath::Matrix4;

use model::Mesh;
use std::sync::Arc;

use state::SceneGraph;

// Represents the public interface for mods
// traits for implementing behavior of state objects should exist here
// but the impls for those traits can be in mods

pub trait Renderer {
    fn init(&mut self) {}
    fn deinit(&mut self) {}

    fn queue_render_layer(&mut self, layer: Arc<SceneGraph>);
    fn present(&mut self);
}

pub trait Identifyable {
    fn identify(&self) -> u64;
}

pub trait Renderable : Identifyable {
    fn get_mesh(&self) -> &Mesh;
    fn get_view_matrix(&self) -> &Matrix4<f32>;
    fn get_world_matrix(&self) -> &Matrix4<f32>;
}
