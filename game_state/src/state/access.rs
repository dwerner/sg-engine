use super::{
    Renderer,
};
// use ui::view::UIView;

use std::collections::VecDeque;
use std::sync::Arc;

use input::InputSource;
use input::events::InputEvent;
use ui::events::UIEvent;

use Identity;

use state::{
    State,
    SimulationLayer,
    SceneGraph,

};

// Accessor trait for State by topic
pub trait RenderAccess {
    fn get_renderers(&mut self) -> &Vec<Box<Renderer>>;
    fn add_renderer(&mut self, renderer: Box<Renderer>);
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
    fn get_input_events(&mut self) -> &mut VecDeque<InputEvent>;
    fn queue_input_event(&mut self, event: InputEvent);

    fn gather_input_events(&mut self);

    fn add_input_source(&mut self, source: Box<InputSource>);
    fn remove_input_source(&mut self, id: Identity);

    fn on_input_load(&mut self);
    fn on_input_unload(&mut self);
}

pub trait SimulationAccess {
    fn get_sim_layers(&mut self) -> &Vec<SimulationLayer>;
    fn on_sim_load(&mut self);
    fn on_sim_unload(&mut self);
}

pub trait UIAccess {
    fn pending_ui_events(&mut self) -> &VecDeque<UIEvent>;
    fn queue_ui_event(&mut self, event: UIEvent);
    fn on_ui_load(&mut self);
    fn on_ui_unload(&mut self);
}


impl RenderLayerAccess for State {

    fn get_render_layers(&mut self) -> &Vec<Arc<SceneGraph>> {
        &mut self.render_layers
    }

    fn add_render_layer(&mut self, layer: Arc<SceneGraph>) {
        self.render_layers.push(layer);
    }

    fn clear_render_layers(&mut self) {
        self.render_layers.clear();
    }

}

impl RenderAccess for State {
    fn get_renderers(&mut self) -> &Vec<Box<Renderer>> {
        &self.renderers
    }

    fn add_renderer(&mut self, renderer: Box<Renderer>) {
        self.renderers.push(renderer);
    }

    fn remove_renderer(&mut self, id: Identity) {
        let mut found = None;
        for i in 0..self.renderers.len() {
            if self.renderers[i].identify() == id {
                found = Some(i as usize);
            }
        }
        if found.is_some() {
            self.renderers.remove(found.unwrap());
        }
    }


    fn on_render_load(&mut self) {
        for i in 0..self.renderers.len() {
            self.renderers[i].load();
        }
        self.push_render_layers();
    }

    fn on_render_unload(&mut self) {
        for i in 0..self.renderers.len() {
            self.renderers[i].unload();
        }
        self.renderers.clear();
    }

    fn clear_renderers(&mut self) {
        self.renderers.clear();
    }

    fn push_render_layers(&mut self) {
        // queue each existing render layers for rendering
        for i in 0..self.renderers.len() {
            for r in &self.render_layers {
                self.renderers[i].queue_render_layer(r.clone());
            }
        }
    }

    fn present_all(&mut self) {
        for i in 0..self.renderers.len() {
            self.renderers[i].present();
        }
    }
}


impl InputAccess for State {

    fn add_input_source(&mut self, source: Box<InputSource>) {
        self.input_state.other_input_sources.push(source);
    }

    fn remove_input_source(&mut self, id: Identity) {
        let mut found = None;
        for i in 0..self.input_state.other_input_sources.len() {
            if self.input_state.other_input_sources[i].identify() == id {
                found = Some(i as usize);
            }
        }
        if found.is_some() {
            self.input_state.other_input_sources.remove(found.unwrap());
        }
    }

    fn gather_input_events(&mut self) {
        // Renderers own the input event loop associated with their
        // internals: i.e. the window manager window
        // - get input events and convert them to our internal format
        // and push them into the input events queue
        // we want to clear that queue each tick, regardless of if we dealt with the events

        // Now we want to
        for i in 0 .. self.renderers.len() {
            let mut events = self.renderers[i].get_input_events();
            if events.len() > 0 {
                self.input_state.pending_input_events.append(&mut events);
            }
        }

        for i in 0..self.input_state.other_input_sources.len() {
            let mut events = self.input_state.other_input_sources[i].get_input_events();
            if events.len() > 0 {
                self.input_state.pending_input_events.append(&mut events);
            }
        }

    }

    fn clear_input_events(&mut self) {
        self.input_state.pending_input_events.clear();
    }
    // Input events might also come from other subsystems, so we allow them to be queued as well
    fn queue_input_event(&mut self, event: InputEvent) {
        self.input_state.pending_input_events.push_back(event);
    }

    fn get_input_events(&mut self) -> &mut VecDeque<InputEvent> {
        &mut self.input_state.pending_input_events
    }
    fn on_input_load(&mut self) {
        self.input_state.clear();
    }

    fn on_input_unload(&mut self) {
        self.input_state.clear();
    }
    fn has_pending_input_events(&self) -> bool {
        !self.input_state.pending_input_events.is_empty()
    }
}

impl SimulationAccess for State {
    fn get_sim_layers(&mut self) -> &Vec<SimulationLayer> {
        &self.simulation_state.layers
    }

    fn on_sim_load(&mut self) {
        unimplemented!();
    }
    fn on_sim_unload(&mut self) {
        unimplemented!();
    }
}

impl UIAccess for State {
    fn pending_ui_events(&mut self) -> &VecDeque<UIEvent> {
        &self.gui_state.pending_ui_events
    }

    fn queue_ui_event(&mut self, event: UIEvent) {
        self.gui_state.pending_ui_events.push_back(event);
    }

    fn on_ui_load(&mut self) {
        unimplemented!()
    }

    fn on_ui_unload(&mut self) {
        unimplemented!()
    }
}
