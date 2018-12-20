extern crate aoc2018;

use aoc2018::day3::count_overlapping_inches;
use aoc2018::day3::get_input;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let count = count_overlapping_inches(get_input());

    println!(
        "There are {} inches in overlapping claims (Took {:?})",
        count,
        start.elapsed()
    );
}
