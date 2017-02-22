pub mod net;
pub mod physics;
pub mod renderer;
pub mod game;

#[macro_use]
extern crate vulkano;

extern crate winit;
extern crate vulkano_win;
extern crate glsl_to_spirv;
extern crate vulkano_shaders;
extern crate cgmath;
extern crate bincode;
extern crate rustc_serialize;

extern crate libloading;
extern crate subproject;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::path::Path;
use std::fs;

use libloading::{Symbol, Library};
use subproject::state;

#[cfg(target_os = "windows")]
const LIBPATH: &'static str = "../../subproject/target/debug/deps/subproject.dll";

#[cfg(target_os = "linux")]
const LIBPATH: &'static str = "subproject/target/debug/deps/libsubproject.so";


pub struct LibLoader {
    lib: Option<Library>,
    modified: Duration,
    version: u64
}

impl LibLoader {
    pub fn new() -> Self {
        let modified = Duration::from_millis(0);
        let mut loader = LibLoader { lib: None, modified: modified, version: 0 };
        loader.check();
        loader
    }

    pub fn check(&mut self) {
        let source = Path::new(LIBPATH);
        let new_meta = fs::metadata(&source).expect("unable to stat file");
        let modified = new_meta.modified().unwrap();
        let duration: Duration = modified.duration_since(UNIX_EPOCH).expect("Unable to get time");
        if self.lib.is_none() || self.modified != duration {
            self.version += 1;
            self.modified = duration;
            println!("Found new library to copy/load at {}", LIBPATH);
            let new_filename = format!("target/libsubproject_{}.so", self.version);
            let copy_result = fs::copy(&source, Path::new(&new_filename));
            match copy_result {
                Ok(_) => println!("copying new lib to {}", new_filename),
                Err(err) => println!("error copying file {}", err)
            }
            match Library::new(&new_filename) {
                Ok(lib) => self.lib = Some(lib),
                Err(err) => println!("Unable to open new library at {} because {}", new_filename, err)
            }
        }
    }

    pub fn func(&self, state: &mut state::State) {
        match self.lib {
            Some(ref lib) => {
                unsafe {
                    let func: Symbol<unsafe extern fn(&mut state::State)> =
                        lib.get(b"use_state\0").expect("unable to find symbol 'use_state'");
                    func(state);
                }
            },
            None => println!("Cannot call method - lib not found")
        }
    }
}
