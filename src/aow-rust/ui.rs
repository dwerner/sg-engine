/*******************************************************************************
 * Copyright 2012 Jannis Tsiroyannis
 *
 * This file is part of Assembly of Worlds.
 *
 * Assembly of Worlds is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Assembly of Worlds is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Assembly of Worlds.  If not, see <http://www.gnu.org/licenses/>.
 *
 *****************************************************************************/

/*******************************************************************************
 * Types
 *
 ******************************************************************************/
pub enum UIElementType {
    NONE_ELEM,
    STEERINGPAD_ELEM,
    THROTTLEPAD_ELEM,
    AFTERBURNER_ELEM,
    ROLL_LEFT_ELEM,
    ROLL_RIGHT_ELEM,

    NEXT_TARGET_ELEM,
    PREVIOUS_TARGET_ELEM,
    NEAREST_ENEMY_ELEM,
    OBJECTIVES_ELEM,

    // Option screen elements
    TOGGLE_INVERT_X_ELEM,
    TOGGLE_INVERT_Y_ELEM,
    TOGGLE_USE_VIB_ELEM,
    TOGGLE_MUSIC_ELEM,
    TOGGLE_VOICEOVER_ELEM,
    TOGGLE_SOUNDFX_ELEM,
    TOGGLE_DRAW_COCKPIT_ELEM,
    TOGGLE_DRAW_BACKDROP_ELEM,
    TOGGLE_DRAW_ENGINE_FLARES_ELEM,
    TOGGLE_AIM_ASSIST_ELEM,
    TOGGLE_DRAW_FPS_ELEM,
    TOGGLE_COLORBLIND_ELEM,
    TOGGLE_FLIP_UI_ELEM,
    TOGGLE_SHOW_UI_ELEM,
    TOGGLE_DRAW_COLLISION_ELEM,

    BACK_BUTTON_ELEM,

    TARGETS_ATTACKER_ELEM,
    TARGET_BOOKMARK_ELEM,
    BOOKMARK_TARGET_ELEM,

    ENTER_HANGAR_ELEM,
    ENTER_HYPERSPACE_ELEM,
    FIRE_EMP_ELEM,
    FIRE_PLASMA_ELEM,

    SKIP_AHEAD_ELEM,
    START_TUTORIAL_ELEM,
    START_MISSION_ELEM,
    CONTINUE_ELEM,
    BRIEFING_ELEM,
    OPTIONS_ELEM,
    SHOWCASE_ELEM,
    CREDITS_ELEM,

    NEXT_MISSION_ELEM,
    PREVIOUS_MISSION_ELEM,
}

pub enum UIElementState {
    NORMAL_ELEM_STATE = 0,
    HIDDEN_ELEM_STATE,
    FLASHING_ELEM_STATE,
}

pub struct UIElement {
    x: f32,
    y: f32,
    sizeX: f32,
    sizeY: f32,
    state: UIElementState,
    uitype: UIElementType, // type is a keyword in rust
    next: Option<Box<UIElement>>,
    flashState: i32, // 1 or 0, flip when "flashing"
    flashFlippedAt: u64
}
