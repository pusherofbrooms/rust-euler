/// Project Euler problems in rust.
mod pr1;
mod pr2;
mod pr3;
mod pr4;
mod pr5;

fn main() {
    println!("Problem 1 answer: {}", pr1::run());
    println!("Problem 2 answer: {}", pr2::run());
    println!("Problem 3 answer: {}", pr3::run());
    println!("Problem 4 answer: {}", pr4::run());
    println!("Problem 5 answer: {}", pr5::run());
}
