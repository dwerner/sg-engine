use libloading::{Symbol, Library};
use ansi_term::Color::Green;
use ansi_term::Color::Yellow;
use game_state::state;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::path::Path;
use std::fs;

use time::PreciseTime;
use time::Duration as TDuration;

pub struct LibLoader {
    filename: String,
    lib: Option<Library>,
    modified: Duration,
    version: u64,
    mod_name: String
}

impl LibLoader {
    pub fn new(filename: &str, mod_name: &str) -> Self {
        let modified = Duration::from_millis(0);
        let mut loader = LibLoader {
            filename: filename.to_string(),
            lib: None,
            modified: modified,
            version: 0,
            mod_name: mod_name.to_string()
        };
        loader
    }

    pub fn check_update(&mut self, state: &mut state::State) {
        let source = Path::new(&self.filename);
        let file_stem = source.file_stem().unwrap().to_str().unwrap();
        match fs::metadata(&source) {
            Ok(new_meta) => {
                let modified = new_meta.modified().unwrap();
                let duration: Duration = modified.duration_since(UNIX_EPOCH).expect("Unable to get time");
                if self.lib.is_none() || self.modified != duration {
                    self.modified = duration;
                    let new_filename = format!("target/{}_{}.so", file_stem, self.version);
                    match fs::copy(&source, Path::new(&new_filename)) {
                        Ok(_) => {
                            if self.lib.is_some() {
                                self.unload(state);
                            }
                            match Library::new(&new_filename) {
                                Ok(lib) => {
                                    self.version += 1;
                                    self.lib = Some(lib);
                                    self.load(state);
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

    // External interface prefers time::Duration (TDuration)
    pub fn tick(&self, state: &mut state::State) -> TDuration {
        let method_name = format!("mod_{}_tick", self.mod_name);
        let start_time = PreciseTime::now();
        self.call(&method_name, state);
        start_time.to(PreciseTime::now())
    }

    fn load(&self, state: &mut state::State) {
        let method_name = format!("mod_{}_load", self.mod_name);
        self.message("Loaded");
        self.call(&method_name, state);
    }

    fn unload(&self, state: &mut state::State) {
        let method_name = format!("mod_{}_unload", self.mod_name);
        self.message("Unloaded");
        self.call(&method_name, state);
    }

    fn message(&self, message: &str) {
        let source = Path::new(&self.filename);
        let file_stem = source.file_stem().unwrap().to_str().unwrap();
        println!( "{}{} {} (version {}){}",
                  Green.bold().paint("["),
                  Green.bold().paint(message),
                  Yellow.paint(file_stem),
                  Yellow.paint(format!("{}",self.version)),
                  Green.bold().paint("]")
        );
    }

    fn call(&self, method_name: &str, state: &mut state::State) {
        match self.lib {
            Some(ref lib) => {
                unsafe {
                    let method = method_name.as_bytes();
                    let func: Symbol<unsafe extern fn(&mut state::State)> =
                        lib.get(method).expect("unable to find symbol");
                    func(state);

                }
            },
            None => println!("Cannot call method {} - lib not found", method_name)
        }
    }
}
