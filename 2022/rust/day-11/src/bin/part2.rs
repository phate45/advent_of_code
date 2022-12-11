use advent_of_code_2022::part2;
use color_eyre::Result;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let file = fs::read_to_string("./real_input.txt").unwrap();
    println!("{}", part2(&file)?);

    Ok(())
}
