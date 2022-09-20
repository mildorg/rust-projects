use std::mem;

pub fn learn() {
    let origin_point = Point::origin();
    let rectangle = Rectangle {
        top_left: origin_point,
        bottom_right: Point::new(3.0, -4.0),
    };

    let boxed_rectangle = Box::new(Rectangle {
        top_left: origin_point,
        bottom_right: Point::new(3.0, -4.0),
    });

    let boxed_origin = Point::boxed_origin();
    let box_in_box = Box::new(Point::boxed_origin());

    println!(
        "origin_point occupies {} bytes on the stack",
        mem::size_of_val(&origin_point)
    );

    println!(
        "rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    println!(
        "boxed_rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );

    println!(
        "boxed_origin occupies {} bytes on the stack",
        mem::size_of_val(&boxed_origin)
    );

    println!(
        "box_in_box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_box)
    );

    let unboxed_point = *boxed_origin;
    println!(
        "unboxed_point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Self> {
        Box::new(Self::origin())
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
