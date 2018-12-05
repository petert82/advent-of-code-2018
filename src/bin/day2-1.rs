extern crate aoc2018;

use aoc2018::day2::calculate_checksum;
use aoc2018::day2::get_input;

fn main() {
    println!("The checksum is {}", calculate_checksum(get_input()));
}