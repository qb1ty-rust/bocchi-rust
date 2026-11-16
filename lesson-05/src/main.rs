#![warn(clippy::all, clippy::pedantic)]

mod generator;

mod prelude {
    pub const LOW: u8 = 0;
    pub const HIGH: u8 = 255;

    pub const LOW_NEG: i8 = - 100;
    pub const HIGH_NEG: i8 = -1;

    pub use crate::generator::{generate, generate_negative};
}

use prelude::*;

fn main() {
    let random_int = generate();
    let random_neg_int = generate_negative();

    println!("Generate random integer: {}", random_int.value);
    println!("Generate random negative integer: {}", random_neg_int.value_negative);
}


pub fn shared() {
    println!("shared");
}