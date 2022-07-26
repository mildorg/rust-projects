use std::convert::From;

#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number { value: n }
    }
}

pub fn practice_from_into() {
    let n = Number::from(30);
    let num: Number = 5.into();

    println!("My number is: {:?}", n);
    println!("My number is: {:?}", num);
}
