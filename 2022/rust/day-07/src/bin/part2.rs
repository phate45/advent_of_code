use color_eyre::Result;
use day_07::part2;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let file = fs::read_to_string("./real_input.txt").unwrap();
    println!("{}", part2(&file)?);
    Ok(())
}
