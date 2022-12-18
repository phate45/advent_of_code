use super::*;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn parse(i: &str) -> IResult<&str, Vec<(Point, Point)>> {
    all_consuming(separated_list1(newline, parse_line))(i.trim_end())
}

fn parse_line(i: &str) -> IResult<&str, (Point, Point)> {
    separated_pair(
        preceded(tag("Sensor at "), parse_point),
        tag(": "),
        preceded(tag("closest beacon is at "), parse_point),
    )(i)
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), nom::character::complete::i32),
            tag(", "),
            preceded(tag("y="), nom::character::complete::i32),
        ),
        |(x, y)| Point(x, y),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let i = "x=2, y=18";
        let r = parse_point(i).unwrap().1;
        assert_eq!(r, Point(2, 18));
    }

    #[test]
    fn test_parse_line() {
        let i = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let r = parse_line(i).unwrap().1;
        assert_eq!(r, (Point(2, 18), Point(-2, 15)));
    }
}
