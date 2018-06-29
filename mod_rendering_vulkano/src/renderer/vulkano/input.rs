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

impl VulkanoRenderer {

    fn handle_event(&mut self, e: &InputEvent) {
        // within the renderer itself we want to listen for input events
        // directly, so we read what's been copied

        match e {
            &InputEvent::KeyDown(ref id, ref scan) =>{
                let c = *scan as u8;
                match c {
                    35 => self.toggle_wrap_cursor(),
                    34 => self.grab_n_hide_cursor(),
                    33 => {
                        self.toggle_fullscreen();
                    }
                    _ => {}
                }
            },
            &InputEvent::MouseMove(_id, ref pos, delta) if self.cursor_wrapped => {
                self.wrap_cursor(pos);
            },
            _ => {}
        }
    }

    fn wrap_cursor(&mut self, pos: &ScreenPoint) {
        if let Some(winit::dpi::LogicalSize{width, height}) = self.window.get_inner_size() {
            if pos.x > (width as i32) - 2 {
                println!("wrapping h");
                println!("{:?}", pos);
                match self.window.set_cursor_position(winit::dpi::LogicalPosition::new(3.0, pos.y as f64)) {
                    Ok(_) => {}
                    Err(e) => println!("unable to set cursor position to {},{}", 3.0, pos.y)
                }
            } else if pos.x < 2 {
                println!("wrapping h");
                println!("{:?}", pos);
                match self.window.set_cursor_position(winit::dpi::LogicalPosition::new(width - 3.0, pos.y as f64)) {
                    Ok(_) => {}
                    Err(e) => println!("unable to set cursor position to {},{}", width - 3.0, pos.y as f64)
                }
            }
            if pos.y > (height as i32) - 2 {
                println!("wrapping v");
                println!("{:?}", pos);
                match self.window.set_cursor_position(winit::dpi::LogicalPosition::new(pos.x as f64, 3.0)) {
                    Ok(_) => {}
                    Err(e) => println!("unable to set cursor position to {},{}", pos.x as f64, 3.0)
                }
            } else if pos.y < 2 {
                println!("wrapping v");
                println!("{:?}", pos);
                match self.window.set_cursor_position(winit::dpi::LogicalPosition::new(pos.x as f64, height - 3.0)) {
                    Ok(_) => {}
                    Err(e) => println!("unable to set cursor position to {},{}", pos.x as f64, height - 3.0)
                }
            }
        }
    }

    fn toggle_wrap_cursor(&mut self) {
        self.cursor_wrapped = !self.cursor_wrapped;
    }

    fn grab_n_hide_cursor(&mut self) {
        let new = !self.cursor_hidden;
        self.window.hide_cursor(new);
        self.cursor_hidden = new;
        match self.window.grab_cursor(new) {
            Ok(_) => {
                println!("{}", if new {"grabbed cursor"} else {"released cursor"});
                self.cursor_grabbed = new;
            },
            Err(e) => println!("unable to grab or release cursor {:?}", e)
        }
    }

    fn toggle_fullscreen(&mut self) {
        let is_fullscreen = !self.fullscreen;
        println!("toggle_fullscreen {} -> {}", self.fullscreen, is_fullscreen);
        // WIN32 WARNING grabbing the cursor and hiding it MUST be done before the set_fullscreen call
        // due to a deadlock in the win32 implementation
        // https://github.com/tomaka/winit/issues/574

        if is_fullscreen {
            let current = self.window.get_current_monitor();
            println!("current monitor {:?}", current);
            self.window.set_fullscreen( Some(current) );
        } else {
            self.window.set_fullscreen(None);
        }
        self.fullscreen = is_fullscreen;
    }

}

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
                    match maybe_converted_event {
                        Some(converted_event) => {
                            // Allow this renderer to peek into and handle specific events immediately
                            self.handle_event(&converted_event);
                            converted_events.push_back(maybe_converted_event.unwrap());

                        },
                        None => {}
                    }

                }
                _ => {}
            };
        }
        converted_events
    }
    // FIXME Ruby
}

