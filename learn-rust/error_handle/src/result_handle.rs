use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

pub fn learn_result() {
    // _open_file();
    // _unwrap_except();
    // let content = _get_file_content();
    let content = _read_file_content();
    match content {
        Ok(c) => println!("{}", c),
        Err(e) => panic!("{}", e),
    }
}

fn _open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("{:?}", &f);
}

fn _unwrap_except() {
    // let f = File::open("helo.txt").unwrap();
    let f = File::open("hello.text").expect("No such file or directory");
    println!("{:?}", &f);
}

fn _get_file_content() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _read_file_content() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
