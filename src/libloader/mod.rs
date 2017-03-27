use libloading::{Symbol, Library};
use ansi_term::Color::{ Green, Yellow, Cyan };
use game_state::state;

use std::time::{UNIX_EPOCH, Duration};
use std::path::Path;
use std::fs;

use time::PreciseTime;
use time::Duration as TDuration;

///
/// Mods support:
///
/// Mods need to be named mod_<mod-name>, and must be unique.
/// Each mod defines a set of extern "C" functions that are called
/// at specific lifecycle points.
///
/// Usage:
/// let a_mod = load_mod!(modnamehere);
/// let mut s = State::new();
/// loop {
///     a_mod.check_update(&mut s);
///     a_mod.tick(&mut s);
/// }
///


///
/// Macro for loading platform-specific shared lib (dll/so)
///
#[macro_export]
macro_rules! load_mod {
	( $s:expr ) => {{
		let name = stringify!($s);
        let path = if cfg!(windows) {
            format!( "mod_{0}/target/debug/mod_{0}.dll", name )
        } else {
            format!( "mod_{0}/target/debug/deps/libmod_{0}.so", name )
        };
		println!("macro load {0} {0}", name);
		LibLoader::new(&path, name)
	}};
}

///
/// LibLoader - an instance represents the managed loading of a dynamic shared library
/// (.dll or .so, potentially in the future a .dylib)
///
/// We keep track of last-modified date of the file, and when it changes we
/// copy the file, along with a version counter to a temporary directory to load it from.
///
pub struct LibLoader {

    filename: String,               // Source filename to watch
    lib:      Option<Library>,
    modified: Duration,
    version:  u64,                  // Keep track of how many times we've loaded,
                                    // as we use this in the filename for the temp copy
    mod_name: String,
}

impl LibLoader {

    ///
    /// Construct a new wrapper for a dynamically loaded mod
    ///
    pub fn new(filename: &str, mod_name: &str) -> Self {
        let modified = Duration::from_millis(0);
        let loader = LibLoader {
            filename: filename.to_string(),
            lib: None,
            modified: modified,
            version: 0,
            mod_name: mod_name.to_string()
        };
        loader
    }

    ///
    /// Check for an update of the lib on disk.
    /// If there has been a change:
    /// - copy it to the tmp directory
    /// - call "unload" lifecycle event on the current mod if there is one
    /// - load the new library
    /// - call "load" lifecycle event on the newly loaded library, passing &mut State
    ///
    pub fn check_update(&mut self, state: &mut state::State) {

        let source = Path::new(&self.filename);
        let file_stem = source.file_stem().unwrap().to_str().unwrap();

        match fs::metadata(&source) {
            Ok(new_meta) => {

                let modified = new_meta.modified()
                    .expect("Unable to retrieve last modified date.");

                let duration: Duration = modified.duration_since(UNIX_EPOCH)
                    .expect("Unable to get time.");

                if self.lib.is_none() || self.modified != duration {

                    self.modified = duration;
                    let new_filename = format!(
                        "target/{}_{}.so",
                        file_stem,
                        self.version
                    );

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
                                Err(err) => println!(
                                    "Unable to open new library: {} - err: {}",
                                    new_filename, err
                                )
                            }
                        },
                        Err(err) => println!(
                            "Error copying file, target: {} - err: {}",
                            new_filename,
                            err
                        )
                    }
                }
            }
            Err(err) => {
                // reset our state to allow any new lib to be loaded if one appears
                if self.lib.is_some() {
                    self.unload(state);
                }
                self.lib = None;
                self.modified = Duration::from_millis(0);
                println!("unable to stat file! {}", err);
            }
        }
    }

    ///
    /// tick()
    ///
    /// Call to the mod to update the state with the "tick" normative lifecycle event
    ///
    // External interface prefers time::Duration (TDuration)
    pub fn tick(&self, state: &mut state::State) -> TDuration {
        let method_name = format!("mod_{}_tick", self.mod_name);
        let start_time = PreciseTime::now();
        self.call(&method_name, state);
        start_time.to(PreciseTime::now())
    }

    ///
    /// load()
    ///
    /// Trigger the "load" lifecycle event
    ///
    fn load(&self, state: &mut state::State) {
        let method_name = format!("mod_{}_load", self.mod_name);
        self.message("Loaded");
        self.call(&method_name, state);
    }

    ///
    /// unload()
    ///
    /// Trigger the unload lifecycle event
    ///
    fn unload(&self, state: &mut state::State) {
        let method_name = format!("mod_{}_unload", self.mod_name);
        self.message("Unloaded");
        self.call(&method_name, state);
    }

    ///
    /// message()
    ///
    /// Print an ansi_term colored message to the terminal.
    /// (used to signal changes in mod versions)
    ///
    fn message(&self, message: &str) {
        let source = Path::new(&self.filename);
        let file_stem = source.file_stem().unwrap().to_str().unwrap();
        println!( "{}{} {} (version {}){}",
                  Green.bold().paint("["),
                  Green.bold().paint(message),
                  Yellow.paint(file_stem),
                  Cyan.paint(format!("{}",self.version)),
                  Green.bold().paint("]")
        );
    }

    ///
    /// call()
    ///
    /// call a method in the module by name, passing &mut State
    ///
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
