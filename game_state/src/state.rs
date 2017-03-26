use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };

use input::events::InputEvent;
use ui::events::UIEvent;
use ui::view::UIView;

use std::collections::VecDeque;
use tree::Node;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use input::screen::ScreenRect;

pub struct State {
    pub renderers: Vec<Box<Renderer>>,
    pub renderables: Vec<Arc<Box<Renderable>>>, // should move this into renderlayers
    //TODO: pub render_layers: Vec<RenderLayer>,

    pub input_state: InputState,
    pub ui_state: UIState,
}

impl State {
    pub fn new(renderers: Vec<Box<Renderer>>) -> Self {
        State{
            renderers: renderers,
            renderables: Vec::new(),
            input_state: InputState {
                pending_input_events: VecDeque::new()
            },
            ui_state: UIState {
                pending_ui_events: VecDeque::new(),
                scene: SceneGraph {
                    root: Node::create(0,UIView::new(0, "id".to_string(), ScreenRect::new(0,0,5,5)), None)
                }
            }
        }
    }
}

pub struct InputState {
    pub pending_input_events: VecDeque<(String, InputEvent)>
}

pub struct SceneGraph<T: Renderable> {
    root: Rc<RefCell<Node<T>>>,
}

pub struct UIState {
    pub pending_ui_events: VecDeque<(String, UIEvent)>,
    pub scene: SceneGraph<UIView>
}
