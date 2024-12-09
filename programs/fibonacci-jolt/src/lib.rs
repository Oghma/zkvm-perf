#![cfg_attr(feature = "guest", no_std)]

use std::hint::black_box;

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}

#[jolt::provable]
pub fn main() {
    let result = black_box(fibonacci(black_box(300000)));
    println!("result: {}", result);
}
