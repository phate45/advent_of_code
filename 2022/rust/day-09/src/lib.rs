mod other;

use indexmap::IndexSet as HashSet;
use color_eyre::Result;
use itertools::Itertools;
use nom::{IResult, combinator::map, sequence::separated_pair, bytes::complete::tag, character::{complete::alpha1, self}};

pub fn part1(source: &str) -> Result<String> {
    let mut grid = Data::new();

    source
        .lines()
        .map(parse_instruction)
        .map(|i| i.unwrap().1)
        .for_each(|i| grid.apply_move(i));

    // dbg!(&grid);

    println!("{:?}", &grid.t_points);
    Ok((grid.t_points.len()).to_string())
}

// TODO fix my solution
#[allow(unreachable_code)]
pub fn part2(source: &str) -> Result<String> {
    return Ok(other::solve_part2(source));
    let mut grid = Data::longer();

    source
        .lines()
        .map(parse_instruction)
        .map(|i| i.unwrap().1)
        .for_each(|i| grid.apply_move(i));

    // dbg!(&grid);

    // println!("{:?}", &grid.t_points);
    Ok((grid.t_points.len()).to_string())
}

#[derive(Debug, Default)]
struct Data {
    rope: Vec<(i32, i32)>,
    t_points: HashSet<(i32, i32)>,
    vis: (u32, i32),
}

impl Data {
    fn new() -> Self {
        let mut s = Self { ..Default::default() };

        for _ in 0..2 {
            s.rope.push((0, 0));
        }

        s.t_points.insert((0, 0));

        s
    }

    fn longer() -> Self {
        let mut s = Self { ..Default::default() };

        for _ in 0..10 {
            s.rope.push((0, 0));
        }

        s.t_points.insert((0, 0));
        s
    }

    fn apply_move(&mut self, m: Instruction) {
        use Direction::*;

        if self.vis.0 > 1 { println!("{m:?}"); };

        for _ in 0..m.1 {
            match &m.0 {
                Right => self.rope[0].1 += 1,
                Left =>  self.rope[0].1 -= 1,
                Up =>    self.rope[0].0 += 1,
                Down =>  self.rope[0].0 -= 1,
            }
            // println!("{}, {}", self.rope[0].0, self.rope[0].1);
            self.update_tail();
            // self.print_rope();
        }
        // println!("{:?}", self.tail());
        // for i in 1..self.rope.len() {
        //     print!("{:?},", Point(self.rope[i-1]) - Point(self.rope[i]));
        // }
        // println!("\n");
        self.visualize();
    }

    fn update_tail(&mut self) {
        use Diff::*;

        for i in 1..self.rope.len() {
           if !self.is_tail_connected(i) {
               let mut new_tail = self.rope[i-1];

               match self.calculate_position(i) {
                   StraightH(s) => {
                       if s > 0 {
                           // right
                           new_tail.1 -= 1;
                       } else {
                           new_tail.1 += 1;
                       }
                   },
                   StraightV(s) => {
                       if s > 0 {
                           // up
                           new_tail.0 -= 1;
                       } else {
                           new_tail.0 += 1;
                       }
                   },
                   KnightH(a) => {
                       if a > 0 {
                           // moved to the right
                           new_tail.1 -= 1;
                       } else {
                           new_tail.1 += 1;
                       }
                   },
                   KnightV(b) => {
                       if b > 0 {
                           // moved up
                           new_tail.0 -= 1;
                       } else {
                           new_tail.0 += 1;
                       }
                   },
                   DiagU(d) => {
                       if d > 0 {
                           // moved right
                           new_tail.1 -= 1;
                           new_tail.0 -= 1;
                       } else {
                           new_tail.1 += 1;
                           new_tail.0 -= 1;
                       }
                   },
                   DiagD(d) => {
                       if d > 0 {
                           // moved right
                           new_tail.1 -= 1;
                           new_tail.0 += 1;
                       } else {
                           new_tail.1 += 1;
                           new_tail.0 += 1;
                       }
                   }
               }

               self.rope[i] = new_tail;

               if i == self.rope.len() - 1 {
                   self.t_points.insert(new_tail);
               }
           }
        }
        // self.print_rope();
    }

    fn is_tail_connected(&self, index: usize) -> bool {
        let i: usize = index - 1;
        let x_range = (self.rope[i].0 - 1)..=(self.rope[i].0 + 1);
        let y_range = (self.rope[i].1 - 1)..=(self.rope[i].1 + 1);

        let t_pos = self.rope[index];
        x_range
            .cartesian_product(y_range)
            .any(|point| point == t_pos)
    }

    fn calculate_position(&self, index: usize) -> Diff {
        use Diff::*;

        let head = self.rope[index - 1];
        let tail = self.rope[index];

        if head.1 == tail.1 {
            return StraightV(head.0 - tail.0);
        } else if head.0 == tail.0 {
            return StraightH(head.1 - tail.1);
        }

        if (head.1 - tail.1).abs() == 2 && (head.0 - tail.0).abs() == 2 {
            // diagonal
            if head.1 > tail.1 {
                // up
                return DiagU(head.0 - tail.0);
            } else {
                return DiagD(head.0 - tail.0);
            }
        }

        // leftovers must be knight
        if (head.1 - tail.1).abs() > 1 {
            // horizontal
            KnightH(head.1 - tail.1)
        } else {
            //vertical
            KnightV( head.0 - tail.0)
        }
    }

    #[allow(dead_code)]
    fn print_rope(&self) {
        println!("{:?}", self.rope.iter().map(|&i| Point(i)).collect::<Vec<Point>>());
    }

    #[allow(dead_code)]
    fn head(&self) -> Point {
        Point(*self.rope.first().unwrap())
    }

    #[allow(dead_code)]
    fn tail(&self) -> Point {
        Point(*self.rope.last().unwrap())
    }

    #[allow(dead_code)]
    fn visualize(&self) {
        if self.vis.0 < 1 {
            return;
        }
        let v = self.vis;
        let mut grid = vec![vec![".".to_string(); (v.0 + v.1 as u32) as usize]; (v.0 + v.1 as u32) as usize];

        self
            .rope
            .iter()
            .enumerate()
            .for_each(|(n, (x, y))| {
                let (x1, y1) = ((*x + v.1) as usize, (*y + v.1) as usize);
                if grid[x1][y1] == "." {
                    let k = if n > 0 {n.to_string()} else { "H".to_string() };
                    grid[x1][y1] = k;
                }
            });

        if grid[v.1 as usize][v.1 as usize] == "." {
            grid[v.1 as usize][v.1 as usize] = "s".to_string();
        }

        for p in &self.t_points {
            grid[(p.0 + v.1) as usize][(p.1 + v.1) as usize] = "#".to_string();
        }

        for v in grid.into_iter().rev() {
            println!("{}", v.into_iter().collect::<String>());
        }
    }
}

/// Positional difference between the head and tail, when they are not touching.
/// Either straight line or knight's move, either horizontal or vertical.
#[derive(Debug, PartialEq)]
enum Diff {
    StraightH(i32),
    StraightV(i32),
    KnightH(i32),
    KnightV(i32),
    DiagU(i32),
    DiagD(i32),
}

#[derive(PartialEq, PartialOrd, Ord, Eq)]
struct Point((i32, i32));
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0.0, self.0.1)
    }
}

impl std::ops::Sub for Point {
    type Output = (i32, i32);

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0.0 - rhs.0.0, self.0.1 - rhs.0.1)
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,

}

#[derive(Debug, PartialEq)]
struct Instruction(Direction, u32);

impl Direction {
    fn from(d: &str) -> Self {
        match d {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unimplemented!("Unknown direction! {}", d),
        }
    }
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        separated_pair(alpha1, tag(" "), character::complete::u32),
        |(d, n)| Instruction(Direction::from(d), n),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "13");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "1");
    }

    #[test]
    // #[ignore]
    fn test_part2_longer() {
        let source = fs::read_to_string("./test_input2.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "36");
    }

    #[test]
    fn test_instruction_parser() {
        let i = parse_instruction("D 3").unwrap();
        assert_eq!(i.1, Instruction(Down, 3));
    }

    fn do_test_movement(d: &mut Data, dd: Direction, n: u32, head_pos: (i32, i32), tail_pos: (i32, i32)) {
        d.apply_move(Instruction(dd, n));

        assert_eq!(d.head(), Point(head_pos));
        assert_eq!(d.tail(), Point(tail_pos));
    }

    #[test]
    fn test_movement() {
        let mut d = Data::new();
        do_test_movement(&mut d, Up, 2, (2, 0), (1, 0));
        do_test_movement(&mut d, Up, 1, (3, 0), (2, 0));

        let mut d = Data::new();
        do_test_movement(&mut d, Right, 2, (0, 2), (0, 1));
        do_test_movement(&mut d, Up, 1, (1, 2), (0, 1));
        do_test_movement(&mut d, Up, 1, (2, 2), (1, 2));

        let mut d = Data::new();
        do_test_movement(&mut d, Right, 2, (0, 2), (0, 1));
        do_test_movement(&mut d, Up, 1, (1, 2), (0, 1));
        do_test_movement(&mut d, Right, 1, (1, 3), (1, 2));
    }

    #[test]
    fn test_diff() {
        let mut d = Data::new();
        d.rope[0] = (0, 2);
        assert_eq!(d.calculate_position(1), Diff::StraightH(2));

        d.rope[0] = (0, -2);
        assert_eq!(d.calculate_position(1), Diff::StraightH(-2));

        d.rope[0] = (2, 0);
        assert_eq!(d.calculate_position(1), Diff::StraightV(2));

        d.rope[0] = (-2, 0);
        assert_eq!(d.calculate_position(1), Diff::StraightV(-2));

        d.rope[0] = (1, 2);
        assert_eq!(d.calculate_position(1), Diff::KnightH(2));

        d.rope[0] = (-1, 2);
        assert_eq!(d.calculate_position(1), Diff::KnightH(2));

        d.rope[0] = (2, 1);
        assert_eq!(d.calculate_position(1), Diff::KnightV(2));

        d.rope[0] = (-2, -1);
        assert_eq!(d.calculate_position(1), Diff::KnightV(-2));

        d.rope[0] = (-2, -2);
        assert_eq!(d.calculate_position(1), Diff::DiagD(-2));

        d.rope[0] = (-2, -1);
        assert_eq!(d.calculate_position(1), Diff::KnightV(-2));
    }

    #[test]
    fn test_moves_smaller() {
        let mut d = Data::longer();
        do_test_movement(&mut d, Right, 4, (0, 4), (0, 0));
        do_test_movement(&mut d, Up, 1, (1, 4), (0, 0));
        assert_eq!(d.rope[1], (0, 3));

        do_test_movement(&mut d, Up, 1, (2, 4), (0, 0));
        assert_eq!(d.rope[1], (1, 4));
        assert_eq!(d.rope[2], (1, 3));

        do_test_movement(&mut d, Up, 2, (4, 4), (0, 0));
        assert_eq!(d.rope[2], (2, 4));
        assert_eq!(d.rope[4], (2, 2));
    }

    #[test]
    #[ignore]
    fn test_visualization() {
        let mut d = Data::longer();
        d.apply_move(Instruction(Right, 4));
        d.vis = (6, 0);
        d.apply_move(Instruction(Up, 3));
        d.apply_move(Instruction(Up, 1));

        assert!(false);
    }

    #[test]
    #[ignore]
    fn test_moves_longer() {
        let mut d = Data::longer();
        d.vis = (25, 11);
        do_test_movement(&mut d, Right, 5, (0, 5), (0, 0));

        d.apply_move(Instruction(Up, 8));
        assert_eq!(d.rope[0], (8, 5));
        assert_eq!(d.rope[1], (7, 5));
        assert_eq!(d.rope[2], (6, 5));
        assert_eq!(d.rope[3], (5, 5));
        assert_eq!(d.rope[4], (4, 5));
        assert_eq!(d.rope[5], (4, 4));
        assert_eq!(d.rope[6], (3, 3));
        assert_eq!(d.rope[7], (2, 2));
        assert_eq!(d.rope[8], (1, 1));
        assert_eq!(d.rope[9], (0, 0));
        // do_test_movement(&mut d, Up, 1, (4, 2), (0, 0));

        d.apply_move(Instruction(Left, 8));
        d.apply_move(Instruction(Down, 3));
        d.apply_move(Instruction(Right, 17));
        d.apply_move(Instruction(Down, 10));
        d.apply_move(Instruction(Left, 25));
        d.apply_move(Instruction(Up, 20));
        assert!(false);
    }

}
