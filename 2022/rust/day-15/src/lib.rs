mod parser;

use std::collections::BTreeSet;

use color_eyre::Result;
use nom::Finish;
use parser::*;

pub fn part1(source: &str) -> Result<String> {
    Ok(solve(source, 2000000))
}

pub fn part2(_source: &str) -> Result<String> {
    let res = "";

    Ok(res.to_string())
}

fn solve(source: &str, row: i32) -> String {
    (parse(source)
        .finish()
        .unwrap()
        .1
        .into_iter()
        .map(Sensor::new)
        .flat_map(|s| s.intersection(row))
        .fold(BTreeSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        })
        .len()
        - 1)
    .to_string()
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct Point(i32, i32);

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    fn intersection(&self, y: i32, distance: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for x in (self.0 - distance)..=(self.0 + distance) {
            let candidate = Point(x, y);
            if self.manhattan_distance(&candidate) <= distance {
                points.push(candidate);
            }
        }

        points
    }
}

#[derive(Debug)]
pub struct Sensor {
    position: Point,
    beacon_distance: i32,
}

impl Sensor {
    fn new(from: (Point, Point)) -> Self {
        Self {
            beacon_distance: from.0.manhattan_distance(&from.1),
            position: from.0,
        }
    }

    fn intersection(&self, y: i32) -> Vec<Point> {
        self.position.intersection(y, self.beacon_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(solve(&source, 10), "26");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "1234");
    }
}
