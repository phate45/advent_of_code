use itertools::Itertools;

pub fn part1(source: &str) -> String {
    let r = source
        .lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .expect("no pair had a sum of 2020");

    (r.0 * r.1).to_string()
}


pub fn part2(source: &str) -> String {
    let r = source
        .lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .expect("no triad had a sum of 2020");

    (r.0 * r.1 * r.2).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source), "514579");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source), "241861950");
    }
}
