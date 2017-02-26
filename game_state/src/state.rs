use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };

pub struct State {
    pub renderers: Vec<Box<Renderer>>,
    pub renderables: Vec<Box<Renderable>>,
    pub blob: u64,
}

impl State {}

