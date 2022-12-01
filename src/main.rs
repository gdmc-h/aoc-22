extern crate reqwest;
mod helpers;
mod aoc;

use crate::aoc::*;
 
fn main() {
    println!("Welcome to Advent of Code 2022!");

    let (level_1_1, level_1_2) = level_1::solve();
    println!("AoC 1 - 1, result: {}", level_1_1);
    println!("AoC 1 - 2, result: {}", level_1_2);
}
