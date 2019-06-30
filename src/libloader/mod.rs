use ansi_term::Color::{Cyan, Green, Yellow};
use game_state::state;
use libloading::{Library, Symbol};

use std::fs;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

use std::io::Error;
use time::Duration as TDuration;
use time::PreciseTime;

///
/// TODO: support a dynamically *defined* and dynamically loaded lib
/// --> Load module definitions at runtime, even watch a mod folder and load them based on a def
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
        #[cfg(debug_assertions)]
        fn build_profile() -> &'static str {
            "debug"
        }

        #[cfg(not(debug_assertions))]
        fn build_profile() -> &'static str {
            "release"
        }

        let name = stringify!($s);
        let path = if cfg!(windows) {
            format!("mod_{0}/target/{1}/mod_{0}.dll", name, build_profile())
        } else {
            format!(
                "mod_{0}/target/{1}/deps/libmod_{0}.so",
                name,
                build_profile()
            )
        };
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
    filename: String, // Source filename to watch
    lib: Option<Library>,
    modified: Duration,
    version: u64, // Keep track of how many times we've loaded,
    // as we use this in the filename for the temp copy
    mod_name: String,
}

impl LibLoader {
    ///
    /// Returns the defined name of the module
    ///
    pub fn get_name(&self) -> &str {
        &self.mod_name
    }

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
            mod_name: mod_name.to_string(),
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
                let modified = new_meta
                    .modified()
                    .expect("Unable to retrieve last modified date.");

                let duration: Duration = modified
                    .duration_since(UNIX_EPOCH)
                    .expect("Unable to get time.");

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
                                }
                                Err(err) => println!(
                                    "Unable to open new library: {} - err: {}",
                                    new_filename, err
                                ),
                            }
                        }
                        Err(err) => println!(
                            "Error copying file, target: {} - err: {}",
                            new_filename, err
                        ),
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
    /// update()
    ///
    /// Call to the mod to update the state with the "update" normative lifecycle event
    ///
    // External interface prefers time::Duration (TDuration)
    pub fn update(&self, state: &mut state::State, delta_time: &TDuration) -> TDuration {
        let method_name = format!("mod_{}_update", self.mod_name);
        // todo:
        let start_time = PreciseTime::now();
        self.call_update(&method_name, state, delta_time);
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
        println!(
            "{}{} {} (version {}, {}){}",
            Green.bold().paint("["),
            Green.bold().paint(message),
            Yellow.paint(file_stem),
            Cyan.paint(format!("{}", self.version)),
            Cyan.paint(format!("{:?}", source)),
            Green.bold().paint("]")
        );
    }

    ///
    /// call_update()
    ///
    /// call a method in the module by name, passing &mut State and a delta_time duration
    ///
    fn call_update(&self, method_name: &str, state: &mut state::State, delta_time: &TDuration) {
        match self.lib {
            Some(ref lib) => unsafe {
                let method = method_name.as_bytes();

                let maybe_func: Result<
                    Symbol<unsafe extern "C" fn(&mut state::State, &TDuration)>,
                    Error,
                > = lib.get(method);

                match maybe_func {
                    Ok(func) => func(state, delta_time),
                    Err(e) => println!(
                        "Unable to call function: {} - method does not exist in lib: {:?}",
                        method_name, lib
                    ),
                }
            },
            None => println!("Cannot call method {} - lib not found", method_name),
        }
    }

    fn call(&self, method_name: &str, state: &mut state::State) {
        match self.lib {
            Some(ref lib) => unsafe {
                let method = method_name.as_bytes();

                let maybe_func: Result<Symbol<unsafe extern "C" fn(&mut state::State)>, Error> =
                    lib.get(method);

                match maybe_func {
                    Ok(func) => func(state),
                    Err(e) => println!(
                        "Unable to call function: {} - method does not exist in lib: {:?}",
                        method_name, lib
                    ),
                }
            },
            None => println!("Cannot call method {} - lib not found", method_name),
        }
    }
}
