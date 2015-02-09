/// Project Euler problems in rust.
extern crate num;
// use num::bigint;

mod pr1;
mod pr2;
mod pr3;
mod pr4;
mod pr5;
mod pr6;
mod pr7;
mod pr8;
mod pr9;
mod pr10;
mod pr11;
mod pr12;
mod pr13;
mod pr14;
mod pr15;

fn main() {
    println!("Problem 1 answer: {}", pr1::run());
    println!("Problem 2 answer: {}", pr2::run());
    println!("Problem 3 answer: {}", pr3::run());
    println!("Problem 4 answer: {}", pr4::run());
    println!("Problem 5 answer: {}", pr5::run());
    println!("Problem 6 answer: {}", pr6::run());
    println!("Problem 7 answer: {}", pr7::run());
    println!("Problem 8 answer: {}", pr8::run());
    println!("Problem 9 answer: {}", pr9::run());
    println!("Problem 10 answer: {}", pr10::run());
    println!("Problem 11 answer: {}", pr11::run());
    println!("Problem 12 answer: {}", pr12::run());
    println!("Problem 13 answer: {}", pr13::run());
    println!("Problem 14 answer: {}", pr14::run());
    println!("Problem 15 answer: {}", pr15::run());
}
