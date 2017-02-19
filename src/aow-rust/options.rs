/*******************************************************************************
* Copyright 2015 Neil Petrick
*
* This file is part of Assembly of Worlds.
*
* Assembly of Worlds is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* Assembly of Worlds is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with Assembly of Worlds.  If not, see <http://www.gnu.org/licenses/>.
*
*****************************************************************************/

use std::env;
use std::io::{ Write, BufReader, BufRead };
use std::error::Error;
use std::path::PathBuf;
use std::fs::{ File, OpenOptions };

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode,decode};

use super::ui::UIElement;

#[allow(non_camel_case_types)]
pub mod values {
    pub const INVERT_X_OPTION: u32 =		    (1 <<  0);	// 0x00000001, 1
    pub const INVERT_Y_OPTION: u32 =			(1 <<  1);	// 0x00000002, 2
    pub const USE_VIB_OPTION: u32 = 			(1 <<  2);	// 0x00000004, 4
    pub const MUSIC_OPTION: u32 = 				(1 <<  3);	// 0x00000008, 8
    pub const VOICEOVER_OPTION: u32 =			(1 <<  4);	// 0x00000010, 16
    pub const SOUNDFX_OPTION: u32 =		    	(1 <<  5);	// 0x00000020, 32
    pub const DRAW_COCKPIT_OPTION: u32 = 		(1 <<  6);	// 0x00000040, 64
    pub const DRAW_BACKDROP_OPTION: u32 =		(1 <<  7);	// 0x00000080, 128
    pub const DRAW_ENGINE_FLARES_OPTION: u32 =  (1 <<  8);	// 0x00000100, 256
    pub const AIM_ASSIST_OPTION: u32 =		 	(1 <<  9);	// 0x00000200, 512
    pub const DRAW_FPS_OPTION: u32 =			(1 <<  10);	// 0x00000400, 1024
    pub const COLORBLIND_OPTION: u32 =			(1 <<  11);	// 0x00000800, 2048
    pub const FLIP_UI_OPTION: u32 =		    	(1 <<  12);	// 0x00001000, 4096
    pub const SHOW_UI_OPTION: u32 =		    	(1 <<  13);	// 0x00002000
    pub const DRAW_COLLISION_SHAPES: u32 =		(1 <<  14);	// 0x00004000
}


// TODO: Options struct who's ctor takes a Renderer (also todo)
// -- or that can be rendered otherwise

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
pub struct Options {
    options: u32
}

impl Options {

    pub fn load() -> Result<Self, String> {
        let file = get_options_file();
        let mut reader = BufReader::new(&file);
        println!("Got options file");
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() > 0 {
            let bytes = line.as_bytes();
            let decode_result = decode(&bytes);
            return match decode_result {
                Err(reason) => { // Corrupt contents
                    println!("Could not read useful data from the options file ({}), creating a new one.", reason);
                    let options = Options::new(0);
                    options.save();
                    Ok(options)
                },
                Ok(options) => {
                    println!("Successfully loaded data from options file.");
                    Ok(options)
                }
            }
        } else {
            println!("Could not read useful data from the options file, creating a new one.");
            let options = Options::new(0);
            options.save();
            return Ok(options)
        }
        Err("Unable to load options file.".to_string())
    }

    pub fn save(&self) {
        println!("Saving options file.");
        let mut file = get_options_file();
        let encoded = &encode(&self, SizeLimit::Infinite).unwrap();
        file.write_all(encoded).unwrap();
        file.write_all(b"\n").unwrap();
        file.sync_all().unwrap();
    }

    pub fn new(o: u32) -> Self {
        Options {options:o}
    }

    pub fn has(&self, o: u32) -> bool {
       self.options & o == o
    }

    pub fn toggle(&mut self, o: u32) {
        self.options ^= o;
    }

    pub fn set_options(&mut self, opts: u32) {
        self.options = opts;
    }
    pub fn get_options(&self) -> u32 {
        self.options
    }

    pub fn create_options_state(&self) {
        panic!("I know not yet this magic");
    }
    pub fn step_options(&self) {
        panic!("I know not yet this magic");
    }
    pub fn options_button_pressed(element: &UIElement) {
        panic!("I know not yet this magic");
    }
}

fn get_options_file_path() -> PathBuf {
    let mut exe_path = env::current_exe().unwrap();
    exe_path.pop();
    exe_path.join("options.bin")
}

fn get_options_file() -> File {
    let path = get_options_file_path();
    match OpenOptions::new().
        write(true).
        read(true).
        create(true). // create if it doesn't already exist
        open(&path) {
        Err(why) => panic!("Unable to open options file. {}", why.description()),
        Ok(file) => {
            println!("Opened options file at {}", path.display());
            file
        }
    }
}

#[cfg(test)]
use std::fs::remove_file;

#[cfg(test)]
fn test_options_file() -> PathBuf{
    let path = get_options_file_path();
    if path.as_path().exists() {
        match remove_file(path.as_path()) {
            Err(e) => assert!(false, format!("Err {}", e)),
            Ok(_) => assert!(true)
        }
    }
    path
}

/*
#[test]
fn should_load_or_create_options_file() {
    let path = test_options_file();
    let file = get_options_file();
    assert!(true);
}
*/

#[test]
fn settings_can_be_saved(){
    let path = test_options_file();
    let optval = 4;
    let mut o1 = Options::load().unwrap();
    o1.set_options(4);
    o1.save();

    let o2 = Options::load().unwrap();
    assert!(o1.get_options() == o2.get_options() && o2.get_options() == optval);
}

#[test]
fn settings_are_set_and_retrieveable() {
    let mut opts = Options::new(0);
    opts.toggle(values::AIM_ASSIST_OPTION);
    assert!(opts.has(values::AIM_ASSIST_OPTION));
}

#[test]
fn settings_are_toggleable_while_others_are_preserved() {
    let mut opts = Options::new(0);
    opts.toggle(values::AIM_ASSIST_OPTION);
    opts.toggle(values::INVERT_X_OPTION);
    assert!(opts.has(values::AIM_ASSIST_OPTION) && opts.has(values::INVERT_X_OPTION));
    opts.toggle(values::AIM_ASSIST_OPTION);
    assert!( !opts.has(values::AIM_ASSIST_OPTION) && opts.has(values::INVERT_X_OPTION));
}
