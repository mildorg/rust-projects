#![allow(unused)]

mod borrowing;
mod bounds;
mod coercion;
mod lifetimes;
mod ownership_move;
mod statics;

pub fn learn() {
    // ownership_move::learn();
    // borrowing::learn();
    // lifetimes::learn();
    // bounds::learn();
    // coercion::learn();
    statics::learn();
}
