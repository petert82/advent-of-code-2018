extern crate aoc2018;

use aoc2018::day2::calculate_checksum;
use aoc2018::day2::get_input;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The checksum is {} (Took {:?})", calculate_checksum(get_input()), start.elapsed());
}