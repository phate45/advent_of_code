/// This solution is borrowed from https://github.com/scristobal/advent-of-code/blob/main/day-09/src/lib.rs
/// I will have to compare outputs step by step later to see where mine is wrong.
/// All tests pass and the visualization matches the example .. yet the result is wrong.

use std::collections::HashSet;
use std::fmt::{self, Debug};

use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::newline;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, multi::separated_list0, IResult};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coords([i32; 2]);

impl Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coord")
            .field("x", &self.0[0])
            .field("y", &self.0[1])
            .finish()
    }
}

impl Coords {
    fn mov(&mut self, dir: &Coords) {
        self.0[0] += dir.0[0];
        self.0[1] += dir.0[1];
    }

    fn diff(&self, coord: &Coords) -> Coords {
        Coords([self.0[0] - coord.0[0], self.0[1] - coord.0[1]])
    }

    fn normalize(&mut self) {
        if self.0[0].abs() <= 1 && self.0[1].abs() <= 1 {
            *self = Coords([0, 0]);
        }
        if self.0[0].abs() > 1 {
            self.0[0] /= self.0[0].abs();
        }
        if self.0[1].abs() > 1 {
            self.0[1] /= self.0[1].abs();
        }
    }
}

fn up(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("U"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([0, 1])).collect()))
}

fn down(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("D"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([0, -1])).collect()))
}

fn left(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("L"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([-1, 0])).collect()))
}

fn right(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("R"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([1, 0])).collect()))
}

fn moves(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, moves) = separated_list0(newline, alt((up, down, right, left)))(input)?;
    Ok((input, moves.into_iter().flatten().collect()))
}

struct LongRope([Coords; 10]);

pub fn solve_part2(input: &str) -> String {
    let (_, moves) = moves(input).unwrap();

    moves
        .into_iter()
        .scan(LongRope([Coords([0, 0]); 10]), |state, mov| {
            state.0[0].mov(&mov);

            for l in 0..(state.0.len() - 1) {
                let mut diff = state.0[l].diff(&state.0[l + 1]);
                diff.normalize();
                state.0[l + 1].mov(&diff);
            }

            Some(state.0[9])
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}
