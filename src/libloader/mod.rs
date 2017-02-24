use libloading::{Symbol, Library};
use game_state::state;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::path::Path;
use std::fs;


pub struct LibLoader {
    filename: String,
    lib: Option<Library>,
    modified: Duration,
    version: u64,
    method: String
}

impl LibLoader {
    pub fn new(filename: &str, method: &str) -> Self {
        let modified = Duration::from_millis(0);
        let mut loader = LibLoader {
            filename: filename.to_string(),
            lib: None,
            modified: modified,
            version: 0,
            method: method.to_string()
        };
        loader.check();
        loader
    }

    pub fn check(&mut self) {
        let source = Path::new(&self.filename);
        match fs::metadata(&source) {
            Ok(new_meta) => {
                let modified = new_meta.modified().unwrap();
                let duration: Duration = modified.duration_since(UNIX_EPOCH).expect("Unable to get time");
                if self.lib.is_none() || self.modified != duration {
                    self.modified = duration;
                    println!("Loading new version ({}) of library {}", self.version, self.filename);
                    let new_filename = format!("target/{}_{}.so",
                                               source.file_stem().unwrap().to_str().unwrap(),
                                               self.version);
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
                    let mut method: Vec<u8> = self.method.clone().into_bytes();
                    //method.push(b'\0');
                    let func: Symbol<unsafe extern fn(&mut state::State)> =
                        lib.get(&method).expect("unable to find symbol");
                    func(state);
                }
            },
            None => println!("Cannot call method - lib not found")
        }
    }
}
