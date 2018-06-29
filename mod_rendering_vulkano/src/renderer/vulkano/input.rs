use super::VulkanoRenderer;

use game_state::winit;
use game_state::input::events::{
    InputEvent,
    MouseButton,
};

use game_state::input::screen::{
    ScreenPoint,
    ScreenRect,
    DeltaVector,
};

use std::collections::VecDeque;
use game_state::input::InputSource;

// TODO: probably extract this to game_state

impl InputSource for VulkanoRenderer {
    fn get_input_events(&mut self) -> VecDeque<InputEvent> {

        //println!("get_input_events");
        let mut events = VecDeque::new();
        {
            let event_loop = &mut self.events_loop.lock().unwrap();
            event_loop.poll_events(|e| events.push_back(e.clone()));
        }

        let this_window_id = self.id as u64;
        //test chg

        let mut converted_events = VecDeque::with_capacity(events.len());

        for e in events {

            match e {
                winit::Event::DeviceEvent{device_id, ref event} => {
                    match event {
                        &winit::DeviceEvent::Added => {
                            println!("device added");
                        },
                        &winit::DeviceEvent::Removed => {
                            println!("device removed")
                        },
                        &winit::DeviceEvent::MouseMotion { delta } => { },
                        &winit::DeviceEvent::MouseWheel {delta} => {},
                        &winit::DeviceEvent::Motion { axis, value } => {
                        },
                        &winit::DeviceEvent::Button { button, state } => {},
                        &winit::DeviceEvent::Key(input) => {},
                        &winit::DeviceEvent::Text{codepoint} => {}
                    }
                },
                winit::Event::WindowEvent{ window_id, ref event } => {
                    let maybe_converted_event = match event {
                        // Keyboard Events
                        &winit::WindowEvent::KeyboardInput{device_id, input} => {
                            let e = match input.state {
                                winit::ElementState::Pressed => InputEvent::KeyDown(self.id, input.scancode),
                                winit::ElementState::Released => InputEvent::KeyUp(self.id, input.scancode)
                            };

                            // within the renderer itself we want to listen for input events
                            // directly, so we read what's been copied
                            if let &InputEvent::KeyDown(ref id, ref scan) = &e {
                                let c = *scan as u8;
                                match c {
                                    33 => {
                                        self.fullscreen = !self.fullscreen;
                                        // TODO: monitor parameter
                                        if self.fullscreen {
                                            println!("going into fullscreen");
                                            self.window.set_fullscreen(None);
                                            self.window.hide_cursor(true);
                                            match self.window.grab_cursor(true) {
                                               Ok(_) => println!("grabbed cursor"),
                                               Err(e) => println!("unable to grab cursor {:?}", e)
                                            }
                                        } else {
                                            println!("leaving fullscreen");
                                            self.window.set_fullscreen(None);
                                            self.window.hide_cursor(false);
                                            match self.window.grab_cursor(false) {
                                                Ok(_) => println!("grabbed cursor"),
                                                Err(e) => println!("unable to grab cursor {:?}", e)
                                            }
                                        }

                                        //self.window.hide_cursor(self.fullscreen);

                                    }
                                    _ => {}
                                }
                            }

                            Some(e)
                        }

                        // Mouse Events
                        &winit::WindowEvent::CursorMoved{device_id, position, modifiers} => {
                            let winit::dpi::LogicalPosition{x,y} = position;
                            let old_pos: ScreenPoint = self.get_mouse_pos().clone();
                            // TODO: resolve f64 truncation to i32 here
                            let new_pos = ScreenPoint::new(x as i32, y as i32);
                            let moved =
                                InputEvent::MouseMove(self.id, new_pos.clone(), DeltaVector::from_points(&old_pos, &new_pos));
                            self.set_mouse_pos(new_pos);
                            Some(moved)
                        },
                        &winit::WindowEvent::MouseInput{device_id, state, button, modifiers} => {
                            let b = match button {
                                winit::MouseButton::Left => MouseButton::Left,
                                winit::MouseButton::Right => MouseButton::Right,
                                winit::MouseButton::Middle => MouseButton::Middle,
                                winit::MouseButton::Other(n) => MouseButton::Other(n)
                            };
                            let e = match state {
                                winit::ElementState::Pressed => InputEvent::MouseDown(self.id, b, self.get_mouse_pos().clone()),
                                winit::ElementState::Released => InputEvent::MouseUp(self.id, b, self.get_mouse_pos().clone())
                            };
                            Some(e)
                        },

                        &winit::WindowEvent::MouseWheel{device_id, delta, phase, modifiers} => {
                            let e = match delta {
                                winit::MouseScrollDelta::LineDelta(x,y) => {
                                    InputEvent::MouseWheel(
                                        self.id, self.get_mouse_pos().clone(),
                                        DeltaVector::new(x as i32, y as i32)
                                    )
                                },
                                winit::MouseScrollDelta::PixelDelta(winit::dpi::LogicalPosition{x,y}) => {
                                    InputEvent::MouseWheel(
                                        self.id, self.get_mouse_pos().clone(),
                                        DeltaVector::new(x as i32, y as i32)
                                    )
                                }
                            };

                            Some(e)
                        },

                        // Window Manager events
                        &winit::WindowEvent::CloseRequested => Some(InputEvent::CloseRequested(self.id)),
                        &winit::WindowEvent::Destroyed => Some(InputEvent::Destroyed(self.id)),
                        &winit::WindowEvent::Focused(f) => Some(if f { InputEvent::GainedFocus(self.id) } else { InputEvent::LostFocus(self.id) }),
                        &winit::WindowEvent::Moved(winit::dpi::LogicalPosition{x,y}) => {
                            let new_rect = ScreenRect::new(x as i32, y as i32, self.rect.w, self.rect.h);
                            let e = InputEvent::Moved(self.id, ScreenPoint::new(x as i32, y as i32));
                            self.set_rect(new_rect);
                            Some(e)
                        }
                        &winit::WindowEvent::Resized(winit::dpi::LogicalSize{width, height}) => {
                            let new_rect = ScreenRect::new(self.rect.x, self.rect.y, width as i32, height as i32);
                            let e = InputEvent::Resized(self.id, new_rect.clone());
                            self.set_rect(new_rect);
                            Some(e)
                        },
                        _ => None

                    };
                    if maybe_converted_event.is_some() {
                        converted_events.push_back(maybe_converted_event.unwrap());
                    }

                }
                _ => {}
            };
        }
        converted_events
    }
    // FIXME Ruby
}

