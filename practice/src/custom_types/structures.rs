#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn practice_structures() {
    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {} and {}", pair.0, pair.1);
    println!();

    let Pair(integer, decimal) = pair;
    println!("pair contains {} and {}", integer, decimal);

    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 2.0 },
        bottom_right: Point { x: 5.0, y: 0.0 },
    };

    println!("The area of react is {}", rect_area(&rect));
    println!();

    let point = Point { x: 0.0, y: 5.0 };
    let square = square(&point, 5f32);
    println!("The square is {:?}", square);
    println!("The area of square is {}", rect_area(&square));
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left,
        bottom_right,
    } = rect;

    (top_left.y - bottom_right.y) * (bottom_right.x - top_left.x)
}

fn square(top_left: &Point, len: f32) -> Rectangle {
    let Point { x, y } = top_left;

    let bottom_right = Point {
        x: x + len,
        y: y - len,
    };

    Rectangle {
        bottom_right,
        top_left: Point { x: *x, y: *y },
    }
}
