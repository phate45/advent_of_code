use std::collections::BTreeSet;

pub fn part1(source: &str) -> String {
    find_start_distinct(source, 4).to_string()
}

pub fn part2(source: &str) -> String {
    find_start_distinct(source, 14).to_string()
}

fn find_start_distinct(source: &str, window: usize) -> usize {
    source
        .as_bytes()
        .windows(window)
        .enumerate()
        .find(|(_i, s)| {
            let set = s.iter().collect::<BTreeSet<&u8>>();
            s.len() == set.len()
        })
        .unwrap().0 + window
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let i = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(i), "5");

        let i = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(i), "6");

        let i = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(i), "10");

        let i = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(i), "11");
    }

    #[test]
    fn test_part2() {
        let i = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part2(i), "19");

        let i = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part2(i), "23");

        let i = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part2(i), "23");

        let i = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part2(i), "29");

        let i = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part2(i), "26");
    }
}
