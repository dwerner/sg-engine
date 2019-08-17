use std::collections::VecDeque;

use crate::ui::events::UIEvent;

#[derive(Default)]
pub struct UIState {
    pub pending_ui_events: VecDeque<UIEvent>,
    //pub scene: SceneGraph
}
