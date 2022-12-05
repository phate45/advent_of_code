pub fn part1(source: &str) -> u32 {
    *add_load(source).iter().max().unwrap()
}

pub fn part2(source: &str) -> u32 {
    let mut load = add_load(source);
    load.sort_unstable();

    load[load.len() - 3..].iter().sum()
}

fn add_load(source: &str) -> Vec<u32> {
    source
        .split("\n\n")
        .map(|load| load.lines().map(|item| item.parse::<u32>().unwrap()).sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1_test() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&file), 24000);
    }

    #[test]
    fn part2_test() {
        let file = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&file), 45000);
    }
}
