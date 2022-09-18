#![allow(unused)]
mod dry;
mod dsl;
mod syntax;

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

pub fn learn() {
    // say_hello!();
    // syntax::learn();
    // dry::learn();
    dsl::learn();
}
