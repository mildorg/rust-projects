use std::fs::{create_dir, File};
use std::path::PathBuf;
use std::{env, io::Write};

use css_minify::optimizations::{Level, Minifier};
use grass;

fn main() {
    let resource = get_absolute_path(vec!["src", "scss", "index.scss"]);
    let resource = resource.to_str().unwrap();

    match grass::from_path(resource, &grass::Options::default()) {
        Ok(content) => write_content(content),
        Err(err) => {
            panic!("Failed to compile styles file due to {:?}", err);
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn get_absolute_path(names: Vec<&str>) -> PathBuf {
    let path_buf = env::current_dir()
        .map(|mut dir| {
            names.iter().for_each(|p| dir.push(p));
            dir
        })
        .unwrap();

    path_buf
}

fn write_content(content: String) {
    create_dest_dir();

    let dest = get_absolute_path(vec!["dist", "mild-style.css"]);
    let dest = dest.as_path();
    let dest_display = dest.display();

    let mut file = match File::create(dest) {
        Ok(file) => file,
        Err(err) => panic!("couldn't create {}: {}", dest_display, err),
    };

    let content = minify(content);
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", dest_display),
        Err(err) => panic!("couldn't wrote to {}: {}", dest_display, err),
    }
}

fn create_dest_dir() {
    let dest_dir = get_absolute_path(vec!["dist"]);
    let dest_dir = dest_dir.as_path();
    let dest_dir_display = dest_dir.display();

    println!("{}", dest_dir_display);

    match dest_dir.read_dir() {
        Ok(_) => return,
        Err(_) => println!("The {} don't exist, create it.", dest_dir_display),
    }

    match create_dir(dest_dir) {
        Ok(_) => println!("successfully created dir: {}", dest_dir_display),
        Err(err) => panic!("couldn't create {}: {}", dest_dir_display, err),
    }
}

fn minify(css: String) -> String {
    Minifier::default().minify(css.as_str(), Level::Two).expect("Failed to minimize css!")
}
