use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error, Read, Write};
use std::path::{Path, PathBuf};

pub fn learn() {
    // test_open_file();
    // test_write_file();
    test_read_lines();
}

fn get_path_buf(file_name: &str) -> PathBuf {
    env::current_dir()
        .map(|dir| {
            let mut dir_path = dir;

            let vec = vec!["src", "stb_misc", file_name];
            vec.into_iter().for_each(|s| dir_path.push(s));

            dir_path
        })
        .unwrap()
}

// open file
fn test_open_file() {
    let path_buf = get_path_buf("hello.txt");
    let path = path_buf.as_path();
    let display = path.display();

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("couldn't open {}: {}", display, err),
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => println!("{} contains: \n{}", display, s),
        Err(err) => panic!("couldn't read {}: {}", display, err),
    }
}

// write file

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn test_write_file() {
    let path_buf = get_path_buf("lorem_ipsum.txt");
    let path = path_buf.as_path();
    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(err) => panic!("couldn't create {}: {}", display, err),
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(err) => panic!("couldn't wrote to {}: {}", display, err),
    }
}

// read lines
fn test_read_lines() -> Result<(), Error> {
    // if let Ok(lines) = read_lines("lorem_ipsum.txt") {
    //     for line in lines {
    //         if let Ok(str) = line {
    //             println!("{}", str);
    //         };
    //     }
    // }

    let lines = read_lines("lorem_ipsum.text")?;

    for line in lines {
        let str = line?;
        println!("{}", str);
    }

    Ok(())
}

fn read_lines(file_name: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let path_buf = get_path_buf(file_name);
    let path = path_buf.as_path();

    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
