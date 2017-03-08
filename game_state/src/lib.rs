pub mod state;
pub mod tree;

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

#[derive(Debug, Clone)]
pub struct ColoredVertex {
    pub position: [f32;3],
    pub color: [f32;4]
}

impl ColoredVertex {
    pub fn new(position: [f32;3], color: [f32;4] ) -> Self {
        ColoredVertex { position: position, color: color}
    }
}

pub trait Renderable : Identifyable {
    fn get_geometry(&self) -> Vec<ColoredVertex>;
}

pub trait Physical : Identifyable {
    fn step(&self);
}

pub trait Syncable : Identifyable {
    fn sync(&self);
}

// renderable
// syncable
// gameobject
// physical

