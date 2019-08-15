use std::collections::VecDeque;

use super::VulkanoRenderer;

use game_state::input::events::{InputEvent, MouseButton};
use game_state::input::screen::{DeltaVector, ScreenPoint, ScreenRect};
use game_state::input::InputSource;
use game_state::winit;

// TODO: probably extract this to game_state
impl VulkanoRenderer {
    // Within the renderer itself we want to listen for input events
    // directly, so we read what's been copied
    fn handle_event(&mut self, e: InputEvent) {
        match e {
            InputEvent::KeyDown(ref _id, c) => match c {
                35 => self.toggle_wrap_cursor(),
                34 => self.grab_n_hide_cursor(),
                33 => {
                    self.toggle_fullscreen();
                }
                _ => {}
            },
            InputEvent::MouseMove(_id, pos, _delta) if self.cursor_wrapped => {
                self.wrap_cursor(pos);
            }
            _ => {}
        }
    }

    // TODO: Doesn't always work in windowed mode. Just ncrease the padding for wrapping in windowed mode?
    // TODO: add threshold param
    fn wrap_cursor(&mut self, pos: ScreenPoint) {
        let winit::dpi::LogicalSize { width, height } = self.window.inner_size();
        if pos.x > (width as i32) - 2 {
            self.window
                .set_cursor_position(winit::dpi::LogicalPosition::new(3.0, f64::from(pos.y)))
                .unwrap_or_else(|_| println!("unable to set cursor position to 3.0,{}", pos.y));
        } else if pos.x < 2 {
            self.window
                .set_cursor_position(winit::dpi::LogicalPosition::new(
                    width - 3.0,
                    f64::from(pos.y),
                ))
                .unwrap_or_else(|_| {
                    println!("unable to set cursor position to {},{}", width - 3.0, pos.y)
                });
        }
        if pos.y > (height as i32) - 2 {
            self.window
                .set_cursor_position(winit::dpi::LogicalPosition::new(pos.x.into(), 3.0))
                .unwrap_or_else(|_| println!("unable to set cursor position to {},3.0", pos.x));
        } else if pos.y < 2 {
            self.window
                .set_cursor_position(winit::dpi::LogicalPosition::new(
                    f64::from(pos.x),
                    height - 3.0,
                ))
                .unwrap_or_else(|_| {
                    println!(
                        "unable to set cursor position to {},{}",
                        f64::from(pos.x),
                        height - 3.0
                    )
                });
        }
    }

    fn toggle_wrap_cursor(&mut self) {
        self.cursor_wrapped = !self.cursor_wrapped;
    }

    fn grab_n_hide_cursor(&mut self) {
        let new = !self.cursor_hidden;
        self.window.set_cursor_visible(new);
        self.cursor_hidden = new;
        match self.window.set_cursor_grab(new) {
            Ok(_) => {
                println!(
                    "{}",
                    if new {
                        "grabbed cursor"
                    } else {
                        "released cursor"
                    }
                );
                self.cursor_grabbed = new;
            }
            Err(e) => println!("unable to grab or release cursor {:?}", e),
        }
    }

    // WIN32 WARNING grabbing the cursor and hiding it MUST be done before the set_fullscreen call
    // due to a deadlock in the win32 implementation - https://github.com/tomaka/winit/issues/574
    fn toggle_fullscreen(&mut self) {
        let is_fullscreen = !self.fullscreen;
        println!("toggle_fullscreen {} -> {}", self.fullscreen, is_fullscreen);
        if is_fullscreen {
            let current = self.window.current_monitor();
            println!("current monitor {:?}", current);
            self.window.set_fullscreen(Some(current));
        } else {
            self.window.set_fullscreen(None);
        }
        self.fullscreen = is_fullscreen;
    }
}

// TODO: simply From<winit::Event> for InputEvent
impl InputSource for VulkanoRenderer {
    fn get_input_events(&mut self) -> VecDeque<InputEvent> {
        let mut events = VecDeque::new();
        events

        /*
        //println!("get_input_events");
        {

            // TODO inject futures::mpsc::channel and send events over that instead
            // of using "get_input_events"

            let el = &mut *self.events_loop.lock().unwrap();
            let mut event_loop = std::mem::replace(el, None);

            event_loop = event_loop.map(|l| {
                l.run(|e, _, control_flow| {
                    events.push_back(e.clone());
                    *control_flow = winit::event_loop::ControlFlow::Poll;
                })
            });

            std::mem::replace(el, event_loop);
        }

        let this_window_id = self.id as u64;
        //test chg

        let mut converted_events = VecDeque::with_capacity(events.len());

        for e in events {
            match &e {
                // TODO: examine if we should be using DeviceEvent as our input rather than WindowEvent
                // this would prevent the need to wrap the cursor when grabbed
                winit::event::Event::DeviceEvent {
                    device_id,
                    ref event,
                } => match event {
                    winit::event::DeviceEvent::Added => {
                        println!("device added  {:?} {:?}", device_id, event);
                    }
                    winit::event::DeviceEvent::Removed => {
                        println!("device removed {:?} {:?}", device_id, event);
                    }
                    _ => {} /*
                            winit::DeviceEvent::MouseMotion { delta } => {}
                            winit::DeviceEvent::MouseWheel { delta } => {}
                            winit::DeviceEvent::Motion { axis, value } => {}
                            winit::DeviceEvent::Button { button, state } => {}
                            winit::DeviceEvent::Key(_input) => {}
                            winit::DeviceEvent::Text { codepoint } => {}
                            */
                },
                winit::event::Event::WindowEvent {
                    window_id,
                    ref event,
                } => {
                    let maybe_converted_event = match event {
                        // Keyboard Events
                        winit::event::WindowEvent::KeyboardInput { device_id, input } => {
                            let e = match input.state {
                                winit::event::ElementState::Pressed => {
                                    InputEvent::KeyDown(self.id, input.scancode)
                                }
                                winit::event::ElementState::Released => {
                                    InputEvent::KeyUp(self.id, input.scancode)
                                }
                            };
                            Some(e)
                        }

                        // Mouse Events
                        winit::event::WindowEvent::CursorMoved {
                            device_id,
                            position,
                            modifiers,
                        } => {
                            let winit::dpi::LogicalPosition { x, y } = position;
                            let old_pos: ScreenPoint = *self.get_mouse_pos();
                            // TODO: resolve f64 truncation to i32 here
                            let new_pos = ScreenPoint::new(*x as i32, *y as i32);
                            let moved = InputEvent::MouseMove(
                                self.id,
                                new_pos,
                                DeltaVector::from_points(&old_pos, &new_pos),
                            );
                            self.set_mouse_pos(new_pos);
                            Some(moved)
                        }

                        winit::event::WindowEvent::MouseInput {
                            device_id,
                            state,
                            button,
                            modifiers,
                        } => {
                            let b = match button {
                                winit::event::MouseButton::Left => MouseButton::Left,
                                winit::event::MouseButton::Right => MouseButton::Right,
                                winit::event::MouseButton::Middle => MouseButton::Middle,
                                winit::event::MouseButton::Other(n) => MouseButton::Other(*n),
                            };
                            let e = match state {
                                winit::event::ElementState::Pressed => {
                                    InputEvent::MouseDown(self.id, b, *self.get_mouse_pos())
                                }
                                winit::event::ElementState::Released => {
                                    InputEvent::MouseUp(self.id, b, *self.get_mouse_pos())
                                }
                            };
                            Some(e)
                        }

                        winit::event::WindowEvent::MouseWheel {
                            device_id,
                            delta,
                            phase,
                            modifiers,
                        } => {
                            let e = match delta {
                                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                    InputEvent::MouseWheel(
                                        self.id,
                                        *self.get_mouse_pos(),
                                        DeltaVector::new(*x as i32, *y as i32),
                                    )
                                }
                                winit::event::MouseScrollDelta::PixelDelta(
                                    winit::dpi::LogicalPosition { x, y },
                                ) => InputEvent::MouseWheel(
                                    self.id,
                                    *self.get_mouse_pos(),
                                    DeltaVector::new(*x as i32, *y as i32),
                                ),
                            };

                            Some(e)
                        }

                        // Window Manager events
                        winit::event::WindowEvent::CloseRequested => {
                            Some(InputEvent::CloseRequested(self.id))
                        }
                        winit::event::WindowEvent::Destroyed => {
                            Some(InputEvent::Destroyed(self.id))
                        }
                        winit::event::WindowEvent::Focused(f) => Some(if *f {
                            InputEvent::GainedFocus(self.id)
                        } else {
                            InputEvent::LostFocus(self.id)
                        }),
                        winit::event::WindowEvent::Moved(winit::dpi::LogicalPosition { x, y }) => {
                            let new_rect =
                                ScreenRect::new(*x as i32, *y as i32, self.rect.w, self.rect.h);
                            let e =
                                InputEvent::Moved(self.id, ScreenPoint::new(*x as i32, *y as i32));
                            self.set_rect(new_rect);
                            Some(e)
                        }
                        winit::event::WindowEvent::Resized(winit::dpi::LogicalSize {
                            width,
                            height,
                        }) => {
                            let new_rect = ScreenRect::new(
                                self.rect.x,
                                self.rect.y,
                                *width as i32,
                                *height as i32,
                            );
                            let e = InputEvent::Resized(self.id, new_rect);
                            self.set_rect(new_rect);
                            Some(e)
                        }
                        _ => None,
                    };
                    if let Some(converted_event) = maybe_converted_event {
                        // Allow this renderer to peek into and handle specific events immediately
                        self.handle_event(converted_event);
                        converted_events.push_back(converted_event);
                    }
                }
                _ => {}
            };
        }
        converted_events
            */
    }
}
