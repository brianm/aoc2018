extern crate aoc2018;
use aoc2018::one;
use std::process;

fn main() {
    match one::part2() {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}
