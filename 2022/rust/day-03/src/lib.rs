use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(source: &str) -> String {
    source.lines()
        .map(|line| line.split_into_sets())
        .map(|(a, b)| {
            a.intersection(&b)
                .map(|c| c.as_priority())
                .sum::<u32>()
        })
        .sum::<u32>()
        .to_string()
}


pub fn part2(source: &str) -> String {
    source.lines()
        .chunks(3)
        .into_iter()
        .map(|a| a.collect::<Vec<&str>>())
        .map(|v| {
            v.iter().map(|s| {
                let c = s.chars().collect::<Vec<char>>();
                uniq!(c)
            })
            .collect::<Vec<HashSet<char>>>()
        })
        .map(|mut v| {
            let mut res = v.pop().unwrap();

            res.retain(|i| {
                v.iter().all(|set| set.contains(i))
            });

            res
        })
        .map(|set| set.iter().map(|c| c.as_priority()).sum::<u32>())
        .sum::<u32>()
        .to_string()
}


trait Priority {
    const LOW_BOUND: (u32, u32) = ('a' as u32, 'z' as u32);
    const HIGH_BOUND: (u32, u32) = ('A' as u32, 'Z' as u32);

    fn as_priority(&self) -> u32;
}

impl Priority for char {

    fn as_priority(&self) -> u32 {
        let value = *self as u32;

        if value >= <Self as Priority>::LOW_BOUND.0 && value <= <Self as Priority>::LOW_BOUND.1 {
            return value - 96;
        }

        if value >= <Self as Priority>::HIGH_BOUND.0 && value <= <Self as Priority>::HIGH_BOUND.1 {
            return value - 38;
        }

        unimplemented!("Unknown char!")
    }
}

trait SplitIntoSets {
    fn split_into_sets(&self) -> (HashSet<char>, HashSet<char>);
}

impl SplitIntoSets for str {
    fn split_into_sets(&self) -> (HashSet<char>, HashSet<char>) {
        debug_assert_eq!(self.len() % 2, 0);

        let mut v = self.chars().collect::<Vec<char>>();
        let v2 = v.split_off(self.len()/2);

        (uniq!(v), uniq!(v2))
    }
}

#[macro_export]
macro_rules! uniq {
    () => { ::std::collections::HashSet::new() };
    ( $v:expr ) => {{
        let mut out = $crate::uniq!();
        for i in $v {
            out.insert(i);
        }
        out
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source), "157");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source), "70");
    }

    #[test]
    fn test_set_split() {
        let s = "abcd";

        assert_eq!(s.split_into_sets(), (HashSet::from(['a', 'b']), HashSet::from(['c', 'd'])));
    }

    #[test]
    fn test_char_conversion () {
        assert_eq!('a' as u32 - 96, 1);
        assert_eq!('z' as u32 - 96, 26);
        assert_eq!('A' as u32 - 38, 27);
        assert_eq!('Z' as u32 - 38, 52);

        assert_eq!('a'.as_priority(), 1);
        assert_eq!('b'.as_priority(), 2);
        assert_eq!('z'.as_priority(), 26);
        assert_eq!('A'.as_priority(), 27);
        assert_eq!('B'.as_priority(), 28);
        assert_eq!('Z'.as_priority(), 52);
    }
}
