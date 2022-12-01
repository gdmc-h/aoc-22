extern crate reqwest;
mod helpers;
mod aoc;

use crate::aoc::level_1::*;
 
fn main() {
    println!("Welcome to Advent of Code 2022");
    println!("AoC 1 - 1, result: {}", level01_part1());
    println!("AoC 1 - 2, result: {}", level01_part2());
}
