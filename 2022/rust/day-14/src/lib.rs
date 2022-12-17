mod aside;

use aside::*;
use color_eyre::Result;
use itertools::{Itertools, MinMaxResult};
use nom::Finish;
use std::fmt;

pub fn part1(source: &str) -> Result<String> {
    let parsed = parse(source).finish().unwrap().1;
    let mut cave = Cave::with_rocks(parsed);

    Ok(cave.calculate_sand().to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let parsed = parse(source).finish().unwrap().1;
    let mut cave = Cave::with_rocks(parsed);

    cave.grid.push(vec![Point::Rock; cave.grid[0].len()]);

    Ok(cave.calculate_sand().to_string())
}

#[derive(Default)]
pub struct Cave {
    sand_source: Coord,
    offset: usize,
    max: usize,
    extra: usize,
    grid: Vec<Vec<Point>>,
}

impl Cave {
    fn with_rocks(rocks: Vec<Vec<Coord>>) -> Self {
        let mut cave = Cave {
            ..Default::default()
        };
        cave.sand_source = Coord(500, 0);
        cave.extra = 500; // padding

        let minmax = rocks.iter().flatten().map(|c| c.0).minmax();

        if let MinMaxResult::MinMax(min, max) = minmax {
            cave.offset = (min).saturating_sub(cave.extra);
            cave.max = max;
        } else {
            unimplemented!("What in tarnation?!")
        }

        cave.add_rocks(rocks);

        cave
    }

    fn calculate_sand(&mut self) -> u32 {
        let mut fallen = 0;
        let mut sand_grain = Sand::new(self);

        'main: loop {
            while let Some(coord) = self.fall(&mut sand_grain) {
                sand_grain.pos_x = coord.0;
                sand_grain.pos_y = coord.1;
            }

            match sand_grain.state {
                SandState::Abyss => break,
                SandState::Rest => {
                    fallen += 1;
                    if sand_grain.as_coord() == self.sand_source {
                        break 'main;
                    }
                    self.rest_sand(sand_grain);
                    sand_grain = Sand::new(self);
                }
                _ => {}
            }
        }

        fallen
    }

    fn fall(&mut self, grain: &mut Sand) -> Option<Coord> {
        use Point::*;

        let x = grain.pos_x;
        let y = grain.pos_y + 1;

        if y == self.grid.len() {
            grain.state = SandState::Abyss;
            return None;
        }

        let space_below = self.get_below(x, y);

        if space_below[1] == &Air {
            return Some(Coord(x, y));
        }

        if space_below[0] == &Air {
            return Some(Coord(x - 1, y));
        }

        if space_below[2] == &Air {
            return Some(Coord(x + 1, y));
        }

        grain.state = SandState::Rest;
        None
    }

    fn rest_sand(&mut self, grain: Sand) {
        self.grid[grain.pos_y][grain.pos_x - self.offset] = Point::Sand;
    }

    fn get_below(&self, x: usize, y: usize) -> Vec<&Point> {
        // need to return the three spaces below the current position
        let x = x - self.offset;
        vec![&self.grid[y][x - 1], &self.grid[y][x], &self.grid[y][x + 1]]
    }

    fn add_rocks(&mut self, rocks: Vec<Vec<Coord>>) {
        // fill the grid with air first

        let depth = rocks.iter().flatten().map(|c| c.1 + 1).max().unwrap();
        for y in 0..=depth {
            self.grid.push(vec![]);
            for _ in 0..=((self.max - self.offset) + self.extra) {
                self.grid[y].push(Point::Air);
            }
        }

        // then add rocks
        for line in rocks {
            line.iter().tuple_windows().for_each(|(left, right)| {
                let diff = *left - *right;
                match (diff.0, diff.1) {
                    (0, _) => {
                        // vertical diff
                        let (y1, y2) = if left.1 > right.1 {
                            (right.1, left.1)
                        } else {
                            (left.1, right.1)
                        };

                        for y in y1..=y2 {
                            self.add_rock(Coord(left.0, y));
                        }
                    }
                    (_, 0) => {
                        // horizontal diff
                        let (x1, x2) = if left.0 > right.0 {
                            (right.0, left.0)
                        } else {
                            (left.0, right.0)
                        };

                        for x in x1..=x2 {
                            self.add_rock(Coord(x, left.1));
                        }
                    }
                    (_, _) => unimplemented!("What in tarnation?!"),
                };
            })
        }
    }

    fn add_rock(&mut self, rock: Coord) {
        self.grid[rock.1][rock.0 - self.offset] = Point::Rock;
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if Coord(x + self.offset, y) == self.sand_source {
                    write!(f, "+")?;
                } else {
                    write!(f, "{:?}", self.grid[y][x])?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cave")
            .field("sand_source", &self.sand_source)
            .field("offset", &self.offset)
            .field("max", &self.max)
            .finish()?;

        // the playing field
        writeln!(f)?;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if Coord(x + self.offset, y) == self.sand_source {
                    write!(f, "+")?;
                } else {
                    write!(f, "{:?}", self.grid[y][x])?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "24");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "93");
    }
}
