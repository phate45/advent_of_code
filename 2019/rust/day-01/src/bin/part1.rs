use day_01::part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./real_input.txt").unwrap();
    println!("{}", part1(&file));
}
