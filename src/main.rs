#![feature(array_chunks, iter_array_chunks)]
extern crate reqwest;
mod helpers;
mod aoc;

use crate::aoc::*;
 
fn main() {
    println!("Welcome to Advent of Code 2022!");

    let (level_1_1, level_1_2) = level_1::solve();
    let (level_2_1, level_2_2) = level_2::solve();
    let (level_3_1, level_3_2) = level_3::solve();
    let (level_4_1, level_4_2) = level_4::solve();

    println!("AoC 1 - 1, result: {}", level_1_1);
    println!("AoC 1 - 2, result: {}", level_1_2);

    println!("AoC 2 - 1, result: {}", level_2_1);
    println!("AoC 2 - 2, result: {}", level_2_2);

    println!("AoC 3 - 1, result: {}", level_3_1);
    println!("AoC 3 - 2, result: {}", level_3_2);

    println!("AoC 4 - 1, result: {}", level_4_1);
    println!("AoC 4 - 2, result: {}", level_4_2);
}
