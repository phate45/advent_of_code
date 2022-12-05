pub fn part1(source: &str) -> String {
    source
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(count_fuel)
        .sum::<u64>()
        .to_string()
}


pub fn part2(source: &str) -> String {
    source
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(count_fuel_recursive)
        .sum::<u64>()
        .to_string()
}

fn count_fuel(mass: u64) -> u64 {
    ((mass as f32 / 3.0).floor() as u64).saturating_sub(2)
}

fn count_fuel_recursive(mass: u64) -> u64 {
    let mut total = 0u64;
    let mut res = count_fuel(mass);

    while res > 0 {
        total += res;
        res = count_fuel(res);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_counter() {
        assert_eq!(count_fuel(12), 2);
        assert_eq!(count_fuel(14), 2);
        assert_eq!(count_fuel(1969), 654);
        assert_eq!(count_fuel(100756), 33583);
    }

    #[test]
    fn test_recursive_counter() {
        assert_eq!(count_fuel_recursive(14), 2);
        assert_eq!(count_fuel_recursive(1969), 966);
        assert_eq!(count_fuel_recursive(100756), 50346);
    }
}
