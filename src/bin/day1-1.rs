extern crate aoc2018;

use aoc2018::day1::get_input;
use aoc2018::day1::sum_lines;
use aoc2018::day1::Error;

fn main() {
    match sum_lines(get_input()) {
        Ok(sum) => println!("Frequency is {}", sum),
        Err(Error::BadNumber) => println!("Bad frequency in input"),
        Err(Error::MissingSign) => println!("Missing sign in input"),
        Err(Error::TooShort) => println!("An input frequency was too short"),
    }
}