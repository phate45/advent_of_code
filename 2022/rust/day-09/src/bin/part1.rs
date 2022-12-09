use color_eyre::Result;
use day_09::part1;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let file = fs::read_to_string("./real_input.txt").unwrap();
    println!("{}", part1(&file)?);

    Ok(())
}
