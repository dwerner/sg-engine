use std::collections::VecDeque;

use ui::events::UIEvent;

pub struct UIState {
    pub pending_ui_events: VecDeque<UIEvent>,
    //pub scene: SceneGraph
}

impl UIState {
    pub fn new() -> Self {
        UIState{ pending_ui_events: VecDeque::new() }
    }
}
