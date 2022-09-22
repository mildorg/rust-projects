use std::path::Path;

pub fn learn() {
    let path = Path::new(".");
    let display = path.display();

    let mut new_path = path.join("a").join("b");

    new_path.push("c");
    new_path.push("my-file.tar.gz");
    new_path.set_file_name("package.tgz");

    match new_path.to_str() {
        Some(s) => println!("new path is {}", s),
        None => println!("new path is not a valid UTF-8 sequence"),
    }
}
