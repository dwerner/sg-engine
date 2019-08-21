use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};

//TODO reexported or implicit?
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use super::Model;
use super::Renderer;
use crate::input::events::InputEvent;
use crate::state::{SceneGraph, State, WindowWithEvents, World};
use crate::ui::events::UIEvent;
use crate::Identity;

pub trait WorldAccess {
    fn get_world(&mut self) -> &mut World;
}

pub trait ModelAccess {
    fn get_models(&self) -> &Vec<Arc<Model>>;
    fn add_model(&mut self, model: Arc<Model>);
}

pub trait WindowAccess {
    fn add_window(&mut self, w: u32, h: u32, title: String);
    fn get_windows(&mut self) -> &Vec<WindowWithEvents>;

    // glium uses a builder rather than a winit::Window... :P
    fn add_window_builder(&mut self, w: f64, h: f64, title: String);
    fn get_window_builders(&self) -> &Vec<WindowBuilder>;
}

// Accessor trait for State by topic
pub trait RenderAccess {
    fn get_renderers(&mut self) -> &Vec<Box<dyn Renderer>>;
    fn add_renderer(&mut self, renderer: Box<dyn Renderer>);
    fn clear_renderers(&mut self);
    fn present_all(&mut self);
    fn remove_renderer(&mut self, id: Identity);
    fn push_render_layers(&mut self);

    fn on_render_load(&mut self);
    fn on_render_unload(&mut self);
}

pub trait RenderLayerAccess {
    fn get_render_layers(&mut self) -> &Vec<Arc<SceneGraph>>;
    fn add_render_layer(&mut self, layer: Arc<SceneGraph>);
    fn clear_render_layers(&mut self);
}

pub trait InputAccess {
    fn has_pending_input_events(&self) -> bool;
    fn clear_input_events(&mut self);
    fn get_input_events(&mut self) -> &VecDeque<InputEvent>;
    fn send_input_event(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>>;
    fn on_input_load(&mut self);
    fn on_input_unload(&mut self);
}

pub trait UIAccess {
    fn pending_ui_events(&mut self) -> &VecDeque<UIEvent>;
    fn queue_ui_event(&mut self, event: UIEvent);
    fn on_ui_load(&mut self);
    fn on_ui_unload(&mut self);
}

impl ModelAccess for State {
    fn get_models(&self) -> &Vec<Arc<Model>> {
        &self.render_state.models
    }

    fn add_model(&mut self, model: Arc<Model>) {
        self.render_state.models.push(model);
    }
}

impl WorldAccess for State {
    fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}

impl WindowAccess for State {
    fn add_window(&mut self, w: u32, h: u32, title: String) {
        let events_loop = Arc::new(Mutex::new(Some(EventLoop::new())));

        let window: Window = {
            let maybe_window = WindowBuilder::new();
            let maybe_window = maybe_window.with_title(title);
            let maybe_window = maybe_window.with_resizable(true);
            let maybe_window = maybe_window.with_inner_size(LogicalSize::new(w.into(), h.into()));
            maybe_window.build(events_loop.lock().unwrap().as_ref().unwrap())
        }
        .expect("unable to create window");

        self.render_state
            .windows
            .push(WindowWithEvents::new(Arc::new(window), events_loop));
    }

    fn get_windows(&mut self) -> &Vec<WindowWithEvents> {
        &self.render_state.windows
    }

    fn add_window_builder(&mut self, w: f64, h: f64, title: String) {
        let maybe_window = WindowBuilder::new();
        let maybe_window = maybe_window.with_title(title);
        let w = maybe_window.with_inner_size(LogicalSize::new(w, h));
        self.render_state.window_builders.push(w);
    }

    fn get_window_builders(&self) -> &Vec<WindowBuilder> {
        &self.render_state.window_builders
    }
}

impl RenderLayerAccess for State {
    fn get_render_layers(&mut self) -> &Vec<Arc<SceneGraph>> {
        &mut self.render_state.render_layers
    }

    fn add_render_layer(&mut self, layer: Arc<SceneGraph>) {
        self.render_state.render_layers.push(layer);
    }

    fn clear_render_layers(&mut self) {
        self.render_state.render_layers.clear();
    }
}

impl RenderAccess for State {
    fn get_renderers(&mut self) -> &Vec<Box<dyn Renderer>> {
        &self.render_state.renderers
    }

    fn add_renderer(&mut self, renderer: Box<dyn Renderer>) {
        self.render_state.renderers.push(renderer);
    }

    fn clear_renderers(&mut self) {
        self.render_state.renderers.clear();
    }

    fn present_all(&mut self) {
        let camera = &self.world.get_facets().cameras[0];
        for r in self.render_state.renderers.iter_mut() {
            r.present(camera);
        }
    }

    fn remove_renderer(&mut self, id: Identity) {
        let mut found = None;
        for i in 0..self.render_state.renderers.len() {
            if self.render_state.renderers[i].identify() == id {
                found = Some(i as usize);
            }
        }
        if found.is_some() {
            self.render_state.renderers.remove(found.unwrap());
        }
    }

    fn push_render_layers(&mut self) {
        // queue each existing render layers for rendering
        for i in 0..self.render_state.renderers.len() {
            for r in &self.render_state.render_layers {
                self.render_state.renderers[i].queue_render_layer(r.clone());
            }
        }
    }

    fn on_render_load(&mut self) {
        for i in 0..self.render_state.renderers.len() {
            self.render_state.renderers[i].load();
        }
    }

    fn on_render_unload(&mut self) {
        for i in 0..self.render_state.renderers.len() {
            self.render_state.renderers[i].unload();
        }
        println!("RenderAccess::on_render_unload");
        self.render_state.renderers.clear();
    }
}

impl InputAccess for State {
    fn has_pending_input_events(&self) -> bool {
        !self.input_state.pending_input_events.is_empty()
    }

    fn clear_input_events(&mut self) {
        self.input_state.pending_input_events.clear();
    }

    fn get_input_events(&mut self) -> &VecDeque<InputEvent> {
        &self.input_state.pending_input_events
    }

    // Input events might also come from other subsystems, so we allow them to be queued as well
    fn send_input_event(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        self.input_state.send(event)
    }

    fn on_input_load(&mut self) {
        self.input_state.clear();
    }

    fn on_input_unload(&mut self) {
        self.input_state.clear();
    }
}

impl UIAccess for State {
    fn pending_ui_events(&mut self) -> &VecDeque<UIEvent> {
        &self.ui_state.pending_ui_events
    }

    fn queue_ui_event(&mut self, event: UIEvent) {
        self.ui_state.pending_ui_events.push_back(event);
    }

    fn on_ui_load(&mut self) {
        unimplemented!()
    }

    fn on_ui_unload(&mut self) {
        unimplemented!()
    }
}
