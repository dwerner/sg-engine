pub mod state;
pub mod tree;
pub mod model;

extern crate cgmath;

// Represents the public interface for mods
// traits for implementing behavior of state objects should exist here
// but the impls for those traits can be in mods

pub trait Renderer {
    // TODO Renderer should take in a SceneGraph
    fn draw(&mut self, renderables: &Vec<Box<Renderable>>);
}

pub trait Identifyable {
    fn identify(&self) -> u64;
}

pub trait Renderable : Identifyable {
    fn get_geometry(&self) -> Vec<model::GVertex>;
}
