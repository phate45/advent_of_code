mod executor;
use executor::*;

pub fn part1(source: &str) -> String {
    let mut e = Executor::new(source);
    e.execute();
    e.output.iter().map(i32::to_string).collect::<String>()
}

pub fn part2(source: &str) -> String {
    let mut e = Executor::with_input(source, "5");
    e.execute();
    e.output.iter().map(i32::to_string).collect::<String>()
}
