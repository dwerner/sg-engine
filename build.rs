// build.rs
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    // from aow-rust
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let dest_dir = get_out_path(&root_dir, "assets/");
    let asset_source_path = Path::new(&root_dir).join("assets");

    println!("Copying assets from: {}", asset_source_path.display());

    let paths = fs::read_dir(asset_source_path).unwrap();

    if !dest_dir.exists() {
        fs::create_dir(dest_dir).expect("unable to create dir");
    }

    for entry in paths {
        let path = entry.unwrap().path();
        if path.is_dir() {
            // TODO: implement recursive copy
        } else {
            let assets = "assets/".to_owned();
            let file_name = assets + path.file_name().unwrap().to_str().unwrap();
            println!("Filename: {}", file_name);
            let dest = get_out_path(&root_dir, &file_name);
            println!("Destionation Path: {}", dest.display());
            match fs::copy(&path, dest) {
                Ok(s) => println!("Copied: {}", s),
                Err(err) => println!("Error: {}", err),
            }
        }

    }
}

fn get_out_path(out_dir: &str, file_name: &str) -> PathBuf {
     let debug = match env::var("DEBUG") {
        Ok(s) => s == "true",
        _ => false
    };
    if debug {
        return Path::new(&out_dir).join("target/debug/").join(file_name);
    }
    
    Path::new(&out_dir).join("target/release/").join(file_name)
}
