use super::VulkanoRenderer;
use std::sync::Arc;

use game_state::input::events::{InputEvent, MouseButton};
use game_state::input::screen::{DeltaVector, ScreenPoint};
use game_state::sdl2;
use game_state::Identity;

// TODO: probably extract this to game_state
impl VulkanoRenderer {
    // Within the renderer itself we want to listen for input events
    // directly, so we read what's been copied
    fn handle_event(&mut self, e: InputEvent) {
        match e {
            InputEvent::KeyDown(ref _id, c) => match c {
                //        35 => self.toggle_wrap_cursor(),
                //        34 => self.grab_n_hide_cursor(),
                33 => {
                    self.toggle_fullscreen();
                }
                _ => {}
            },
            InputEvent::MouseMove(_id, pos) if self.cursor_wrapped => {
                //self.wrap_cursor(pos);
            }
            _ => {}
        }
    }

    // TODO: Doesn't always work in windowed mode. Just ncrease the padding for wrapping in windowed mode?
    // TODO: add threshold param
    fn wrap_cursor(&mut self, pos: ScreenPoint) {
        if let Ok(sdl2::video::DisplayMode {
            format,
            w,
            h,
            refresh_rate,
        }) = self.window.display_mode()
        {
            /*
            if pos.x > (w as i32) - 2 {
                self.window
                    .set_cursor_position(winit::dpi::LogicalPosition::new(3.0, f64::from(pos.y)))
                    .unwrap_or_else(|_| println!("unable to set cursor position to 3.0,{}", pos.y));
            } else if pos.x < 2 {
                self.window
                    .set_cursor_position(winit::dpi::LogicalPosition::new(
                        w - 3.0,
                        f64::from(pos.y),
                    ))
                    .unwrap_or_else(|_| {
                        println!("unable to set cursor position to {},{}", w - 3.0, pos.y)
                    });
            }
            if pos.y > (h as i32) - 2 {
                self.window
                    .set_cursor_position(winit::dpi::LogicalPosition::new(pos.x.into(), 3.0))
                    .unwrap_or_else(|_| println!("unable to set cursor position to {},3.0", pos.x));
            } else if pos.y < 2 {
                self.window
                    .set_cursor_position(winit::dpi::LogicalPosition::new(
                        f64::from(pos.x),
                        h - 3.0,
                    ))
                    .unwrap_or_else(|_| {
                        println!(
                            "unable to set cursor position to {},{}",
                            f64::from(pos.x),
                            h - 3.0
                        )
                    });
            }
            */
        }
    }

    // WIN32 WARNING grabbing the cursor and hiding it MUST be done before the set_fullscreen call
    // due to a deadlock in the win32 implementation - https://github.com/tomaka/winit/issues/574
    fn toggle_fullscreen(&mut self) {
        let is_fullscreen = !self.fullscreen;
        println!("toggle_fullscreen {} -> {}", self.fullscreen, is_fullscreen);
        if is_fullscreen {
            //self.window.set_fullscreen(sdl2::video::FullscreenType::Off);
            //} else {
            //self.window
            //   .set_fullscreen(sdl2::video::FullscreenType::Desktop);
        }
        self.fullscreen = is_fullscreen;
    }

    /*
    pub fn store_events(&self) {
        // HACK: using winit 0.19 to work around "better api" aka winit's hacky event loop
        // https://users.rust-lang.org/t/winit-0-20-the-state-of-windowing-in-rust-and-a-request-for-help/29485/31
        let id = self.id;
        match &mut *self.events_loop.lock().unwrap() {
            Some(ref mut el) => {
                let events = Arc::downgrade(&self.events);
                el.poll_events(|e| {
                    if let Some(events) = events.upgrade() {
                        if let Some(event) = VulkanoRenderer::convert_event(id, &e) {
                            println!("got event {:?}", event);
                            events.lock().unwrap().push_back(event);
                        }
                    }
                });
            }
            None => {
                println!("no event loop");
            }
        }
    }

        fn convert_event(id: Identity, e: &winit::Event) -> Option<InputEvent> {
            match e {
                // TODO: examine if we should be using DeviceEvent as our input rather than WindowEvent
                // this would prevent the need to wrap the cursor when grabbed
                winit::Event::DeviceEvent {
                    device_id,
                    ref event,
                } => match event {
                    winit::DeviceEvent::Added => {
                        println!("device added  {:?} {:?}", device_id, event);
                        None
                    }
                    winit::DeviceEvent::Removed => {
                        println!("device removed {:?} {:?}", device_id, event);
                        None
                    }
                    _ => None, /*
                               winit::DeviceEvent::MouseMotion { delta } => {}
                               winit::DeviceEvent::MouseWheel { delta } => {}
                               winit::DeviceEvent::Motion { axis, value } => {}
                               winit::DeviceEvent::Button { button, state } => {}
                               winit::DeviceEvent::Key(_input) => {}
                               winit::DeviceEvent::Text { codepoint } => {}
                               */
                },
                winit::Event::WindowEvent {
                    window_id,
                    ref event,
                } => {
                    let maybe_converted_event = match event {
                        // Keyboard Events
                        winit::WindowEvent::KeyboardInput { device_id, input } => {
                            let e = match input.state {
                                winit::ElementState::Pressed => InputEvent::KeyDown(id, input.scancode),
                                winit::ElementState::Released => InputEvent::KeyUp(id, input.scancode),
                            };
                            Some(e)
                        }

                        // Mouse Events
                        winit::WindowEvent::CursorMoved {
                            device_id,
                            position,
                            modifiers,
                        } => {
                            let winit::dpi::LogicalPosition { x, y } = position;
                            let new_pos = ScreenPoint::new(*x as i32, *y as i32);
                            let moved = InputEvent::MouseMove(id, new_pos);
                            Some(moved)
                        }

                        winit::WindowEvent::MouseInput {
                            device_id,
                            state,
                            button,
                            modifiers,
                        } => {
                            let b = match button {
                                winit::MouseButton::Left => MouseButton::Left,
                                winit::MouseButton::Right => MouseButton::Right,
                                winit::MouseButton::Middle => MouseButton::Middle,
                                winit::MouseButton::Other(n) => MouseButton::Other(*n),
                            };
                            let e = match state {
                                winit::ElementState::Pressed => InputEvent::MouseDown(id, b),
                                winit::ElementState::Released => InputEvent::MouseUp(id, b),
                            };
                            Some(e)
                        }

                        winit::WindowEvent::MouseWheel {
                            device_id,
                            delta,
                            phase,
                            modifiers,
                        } => {
                            let e = match delta {
                                winit::MouseScrollDelta::LineDelta(x, y) => {
                                    InputEvent::MouseWheel(id, DeltaVector::new(*x as i32, *y as i32))
                                }
                                winit::MouseScrollDelta::PixelDelta(winit::dpi::LogicalPosition {
                                    x,
                                    y,
                                }) => {
                                    InputEvent::MouseWheel(id, DeltaVector::new(*x as i32, *y as i32))
                                }
                            };

                            Some(e)
                        }

                        // Window Manager events
                        winit::WindowEvent::CloseRequested => Some(InputEvent::CloseRequested(id)),
                        winit::WindowEvent::Destroyed => Some(InputEvent::Destroyed(id)),
                        winit::WindowEvent::Focused(f) => Some(if *f {
                            InputEvent::GainedFocus(id)
                        } else {
                            InputEvent::LostFocus(id)
                        }),
                        winit::WindowEvent::Moved(winit::dpi::LogicalPosition { x, y }) => {
                            let e = InputEvent::Moved(id, ScreenPoint::new(*x as i32, *y as i32));
                            Some(e)
                        }
                        winit::WindowEvent::Resized(winit::dpi::LogicalSize { width, height }) => {
                            let e = InputEvent::Resized(id, *width as f32, *height as f32);
                            Some(e)
                        }
                        _ => None,
                    };
                    maybe_converted_event
                }
                _ => None,
            }
        }
    */
}
