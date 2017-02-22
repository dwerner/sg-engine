use libloading::{Symbol, Library};
use subproject::state;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::path::Path;
use std::fs;


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
        match fs::metadata(&source) {
            Ok(new_meta) => {
                let modified = new_meta.modified().unwrap();
                let duration: Duration = modified.duration_since(UNIX_EPOCH).expect("Unable to get time");
                if self.lib.is_none() || self.modified != duration {
                    self.modified = duration;
                    println!("Loading new version ({}) of library {}", self.version, LIBPATH);
                    let new_filename = format!("target/libsubproject_{}.so", self.version);
                    println!("copying new lib to {}", new_filename);
                    match fs::copy(&source, Path::new(&new_filename)) {
                        Ok(_) => {
                            match Library::new(&new_filename) {
                                Ok(lib) => {
                                    self.version += 1;
                                    self.lib = Some(lib);
                                },
                                Err(err) => println!("Unable to open new library at {} because {}", new_filename, err)
                            }
                        },
                        Err(err) => println!("error copying file {}", err)
                    }
                }
            }
            Err(err) => {
                self.lib = None;
                self.modified = Duration::from_millis(0);
                println!("unable to stat file! {}", err);
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
