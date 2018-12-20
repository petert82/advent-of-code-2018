extern crate aoc2018;

use aoc2018::day3::find_intact_claim;
use aoc2018::day3::get_input;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    match find_intact_claim(get_input()) {
        Some(id) => println!(
            "The only intact claim is {} (Took {:?}",
            id,
            start.elapsed()
        ),
        None => println!("There are no intact claims (Took {:?}", start.elapsed()),
    }
}
