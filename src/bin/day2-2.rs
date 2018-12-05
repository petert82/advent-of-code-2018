extern crate aoc2018;

use aoc2018::day2::find_similar_ids;
use aoc2018::day2::get_input;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    match find_similar_ids(get_input()) {
        None => println!("There are no similar IDs in the input(Took {:?})", now.elapsed()),
        Some(similar) => println!("The common characters are: {} (Took {:?})", similar, now.elapsed()),
    }
}