use std::collections::VecDeque;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

use sdl2::video::WindowContext;

use super::DrawMode;
use super::Model;
use super::Renderer;

use crate::input::events::InputEvent;
use crate::input::screen::ScreenPoint;
use crate::state::render_state::WindowWithAttrs;
use crate::state::{SceneGraph, State, World};
use crate::ui::events::UIEvent;
use crate::Identity;

use crate::state::Variable;

pub trait WorldAccess {
    fn get_world(&mut self) -> &mut World;
}

pub trait ModelAccess {
    fn get_models(&self) -> &Vec<Arc<Model>>;
    fn add_model(&mut self, model: Arc<Model>);
}

pub trait VariableAccess {
    fn get_bool(&self, key: &'static str) -> Option<bool>;
    fn set_bool(&mut self, key: &'static str, value: bool);
    fn bool_exists(&self, key: &'static str) -> bool {
        self.get_bool(key).is_some()
    }
}

pub trait WindowAccess {
    fn add_window(&mut self, w: u32, h: u32, title: &str, x: i32, y: i32, draw_mode: DrawMode);
    fn get_windows(&mut self) -> Vec<(Rc<WindowContext>, DrawMode)>;
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
    fn get_mouse_pos(&self) -> &ScreenPoint;
    fn set_mouse_pos(&mut self, sp: ScreenPoint);
}

pub trait UIAccess {
    fn pending_ui_events(&mut self) -> &VecDeque<UIEvent>;
    fn queue_ui_event(&mut self, event: UIEvent);
    fn on_ui_load(&mut self);
    fn on_ui_unload(&mut self);
}

impl VariableAccess for State {
    fn get_bool(&self, key: &'static str) -> Option<bool> {
        self.variables.get(key).map(|Variable::Bool(v)| *v)
    }

    fn set_bool(&mut self, key: &'static str, value: bool) {
        self.variables.insert(key, Variable::Bool(value));
    }
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
    // TODO: make fallible
    fn add_window(&mut self, w: u32, h: u32, title: &str, x: i32, y: i32, draw_mode: DrawMode) {
        let window = {
            self.sdl_subsystems
                .video
                .window(title, w, h)
                .position(x, y)
                .resizable()
                .allow_highdpi()
                .vulkan()
                .build()
                .unwrap()
        };

        self.render_state
            .windows
            .push(WindowWithAttrs { window, draw_mode });
    }

    fn get_windows(&mut self) -> Vec<(Rc<WindowContext>, DrawMode)> {
        self.render_state
            .windows
            .iter()
            .map(|w| (w.window.context(), w.draw_mode))
            .collect::<Vec<_>>()
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
        if let Some(found) = found {
            self.render_state.renderers.remove(found);
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
        !self.input_state.get_input_events().is_empty()
    }

    fn clear_input_events(&mut self) {
        self.input_state.clear();
    }

    fn get_input_events(&mut self) -> &VecDeque<InputEvent> {
        &self.input_state.get_input_events()
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

    fn get_mouse_pos(&self) -> &ScreenPoint {
        self.input_state.get_mouse_pos()
    }

    fn set_mouse_pos(&mut self, sp: ScreenPoint) {
        self.input_state.set_mouse_pos(sp)
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
