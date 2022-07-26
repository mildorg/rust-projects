use std::fmt::{Display, Formatter, Result};

struct Circle {
    radius: u32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

pub fn practice_to_from_string() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("sum is {}", sum);
}
