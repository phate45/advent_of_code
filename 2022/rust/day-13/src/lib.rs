use std::cmp::Ordering;

use color_eyre::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn part1(source: &str) -> Result<String> {
    Ok(pairs(source)
        .expect("The input must be valid!")
        .1
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .filter(|(_, pair)| pair.left.cmp(&pair.right) == Ordering::Less)
        .map(|(i, _)| i)
        .sum::<usize>()
        .to_string())
}

pub fn part2(source: &str) -> Result<String> {
    use Packet::*;

    Ok(pairs(source)
        .expect("The input must be valid!")
        .1
        .into_iter()
        .chain(vec![Pair {
            left: List(vec![List(vec![Number(2)])]),
            right: List(vec![List(vec![Number(6)])]),
        }])
        .flat_map(|p| vec![p.left, p.right])
        .sorted_by(|left, right| match (&left, &right) {
            (List(left), List(right)) => left.cmp(right),
            (Number(left), Number(right)) => left.cmp(right),
            (List(left), Number(right)) => left.cmp(&vec![Number(*right)]),
            (Number(left), List(right)) => vec![Number(*left)].cmp(right),
        })
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .filter(|(_, packet)| {
            packet == &List(vec![List(vec![Number(2)])])
                || packet == &List(vec![List(vec![Number(6)])])
        })
        .map(|(i, _)| i)
        .product::<usize>()
        .to_string())
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        use Packet::*;

        match (self, other) {
            (List(left), List(right)) => left == right,
            (Number(left), Number(right)) => left == right,
            (List(left), Number(right)) => left == &vec![Packet::Number(*right)],
            (Number(left), List(right)) => &vec![Packet::Number(*left)] == right,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;

        match (self, other) {
            (List(left), List(right)) => left.cmp(right),
            (Number(left), Number(right)) => left.cmp(right),
            (List(left), Number(right)) => left.cmp(&vec![Number(*right)]),
            (Number(left), List(right)) => vec![Number(*left)].cmp(right),
        }
    }
}

pub fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(Packet::List),
        nom::character::complete::u32.map(Packet::Number),
    ))(input)
}

pub fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(left, right)| Pair { left, right }),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "13");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "140");
    }

    #[test]
    fn test_parsing() {
        let i = "[1,1,3,1,1]\n[1,1,5,1,1]";
        let p = pairs(i).unwrap();
        // dbg!(p);
        assert_eq!(p.0, "");
        assert!(!p.1.is_empty());
        assert_eq!(
            p.1[0],
            Pair {
                left: Packet::List(vec![
                    Packet::Number(1),
                    Packet::Number(1),
                    Packet::Number(3),
                    Packet::Number(1),
                    Packet::Number(1)
                ]),
                right: Packet::List(vec![
                    Packet::Number(1),
                    Packet::Number(1),
                    Packet::Number(5),
                    Packet::Number(1),
                    Packet::Number(1)
                ]),
            }
        );

        let i = "[[[]]]\n[[]]";
        let p = pairs(i).unwrap();
        assert_eq!(p.0, "");
        assert!(!p.1.is_empty());
        assert_eq!(
            p.1[0],
            Pair {
                left: Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]),
                right: Packet::List(vec![Packet::List(vec![])])
            }
        );
    }
}
