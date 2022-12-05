use day_03::part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./real_input.txt").unwrap();
    println!("{}", part2(&file));
}
