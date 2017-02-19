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


use cgmath::Vector3;

pub const MAX_NUMBER_OF_EXPLOSIONS: usize = 64;
pub const EXPLOSION_TIME: u32 = 800;
pub const LOG_ENTRY_DISPLAY_TIME: u32 = 7000;

pub struct Renderer {
// glium window?
    // gamestate
}

struct Explosion {
    position: Vector3<f32>,
    start_time: u64,
    size: f64
}

impl Renderer {

    pub fn new() -> Self {
        Renderer {}
    }

    pub fn create_rendering_context() {

    }

    pub fn destroy_rendering_context() {

    }

    pub fn render_frame(&self, current_time: u64) {

    }

    pub fn render_hyperspace(&self, current_time: u64) {

    }

    pub fn render_pausescreen(&self, current_time: u64) {

    }

    pub fn reset_rendering_state(&mut self) {

    }

    pub fn render_line_segment(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {

    }

    pub fn add_explosion(postion: &Vector3<f32>, current_time: u64, size: f64) {

    }

    fn render_explosions(&self) {

    }

    fn render_lasers(&self) {

    }

    fn render_crosshairs(&self){}
}