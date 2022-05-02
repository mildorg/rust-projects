use std::fmt::{self, Display};
use std::ops::Add;

pub fn custom_operation() {
    // let p1 = Point {
    //     x: 1.1f32,
    //     y: 1.1f32,
    // };
    // let p2 = Point {
    //     x: 2.1f32,
    //     y: 2.1f32,
    // };

    // println!("{:?}", add(p1, p2));

    // let p3 = Point { x: 1i32, y: 1i32 };
    // let p4 = Point { x: 2i32, y: 2i32 };
    // println!("{:?}", add(p3, p4));

    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}
