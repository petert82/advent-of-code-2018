extern crate aoc2018;

use aoc2018::day1::find_repeat;
use aoc2018::day1::get_input;
use aoc2018::day1::Error;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    match find_repeat(get_input()) {
        Ok(freq) => println!("First repeated frequency is {} (Took {:?})", freq, start.elapsed()),
        Err(Error::BadNumber) => println!("Bad frequency in input (Took {:?})", start.elapsed()),
        Err(Error::MissingSign) => println!("Missing sign in input (Took {:?})", start.elapsed()),
        Err(Error::TooShort) => println!("An input frequency was too short (Took {:?})", start.elapsed()),
    }
}