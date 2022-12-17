use super::*;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Default, Debug, PartialEq)]
pub enum SandState {
    #[default]
    Falling,
    Rest,
    Abyss,
}

#[derive(Debug)]
pub struct Sand {
    // cave: &'a Cave,
    pub state: SandState,
    pub pos_x: usize,
    pub pos_y: usize,
}

impl Sand {
    pub fn new(cave: &Cave) -> Self {
        Self {
            // cave,
            state: SandState::default(),
            pos_x: cave.sand_source.0,
            pos_y: cave.sand_source.1,
        }
    }

    pub fn as_coord(&self) -> Coord {
        Coord(self.pos_x, self.pos_y)
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Point {
    #[default]
    Air,
    Rock,
    Sand,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Coord(pub usize, pub usize);

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Self) -> Self::Output {
        Coord(self.0.abs_diff(other.0), self.1.abs_diff(other.1))
    }
}

pub fn parse(i: &str) -> IResult<&str, Vec<Vec<Coord>>> {
    all_consuming(separated_list1(newline, parse_line))(i.trim_end())
}

fn parse_line(i: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(tag(" -> "), parse_coord)(i)
}

fn parse_coord(i: &str) -> IResult<&str, Coord> {
    map(
        separated_pair(
            map(nom::character::complete::u32, |n| n as usize),
            tag(","),
            map(nom::character::complete::u32, |n| n as usize),
        ),
        |(x, y)| Coord(x, y),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_calc() {
        let i = "498,4 -> 498,6 -> 496,6";
        let r = parse_line(i).unwrap().1;

        let offset = r.iter().map(|c| c.0).min().unwrap().to_owned();

        assert_eq!(offset, 496);
    }

    #[test]
    fn test_parse_line() {
        let i = "498,4 -> 498,6 -> 496,6";
        let r = parse_line(i).unwrap().1;
        assert_eq!(r, vec![Coord(498, 4), Coord(498, 6), Coord(496, 6)]);
    }

    #[test]
    fn test_parse_coord() {
        let i = "123,4";
        let r = parse_coord(i).unwrap().1;
        assert_eq!(r, Coord(123, 4));
    }
}
