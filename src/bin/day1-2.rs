extern crate aoc2018;

use aoc2018::day1::find_repeat;
use aoc2018::day1::get_input;
use aoc2018::day1::Error;

fn main() {
    match find_repeat(get_input()) {
        Ok(freq) => println!("First repeated frequency is {}", freq),
        Err(Error::BadNumber) => println!("Bad frequency in input"),
        Err(Error::MissingSign) => println!("Missing sign in input"),
        Err(Error::TooShort) => println!("An input frequency was too short"),
    }
}