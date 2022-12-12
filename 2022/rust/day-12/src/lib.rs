use color_eyre::Result;
use pathfinding::prelude::astar;

pub fn part1(source: &str) -> Result<String> {
    let (grid, start, end) = parse_grid(source);

    let res = solve(&grid, &start, &end);

    let (path, steps) = res.unwrap();
    show(path, &grid);
    Ok(steps.to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let (grid, _, end) = parse_grid(source);

    let res = grid
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, v)| *v == &1u16)
                .map(|(x, _)| solve(&grid, &Pos(x, y), &end))
                .collect::<Vec<Option<(Vec<Pos>, u32)>>>()
        })
        .flatten()
        .min_by_key(|(_, steps)| *steps)
        .unwrap();

    let (path, steps) = res;
    show(path, &grid);
    Ok(steps.to_string())
}

fn solve(grid: &[Vec<u16>], start: &Pos, end: &Pos) -> Option<(Vec<Pos>, u32)> {
    astar(
        start,
        |p| p.successors(grid),
        |p| p.distance(end),
        |p| p == end,
    )
}

fn show(path: Vec<Pos>, grid: &[Vec<u16>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if path.contains(&Pos(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, grid: &[Vec<u16>]) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let val = grid[y][x];

        self.neighbors(x, y, grid)
            .into_iter()
            .filter(|(ix, iy)| Pos(*ix, *iy) != *self)
            .filter(|(ix, iy)| {
                grid[*iy][*ix] <= (val + 1)
            })
            .map(|(ix, iy)| (Pos(ix, iy), 1))
            .collect()
    }

    fn neighbors(&self, x: usize, y: usize, grid: &[Vec<u16>]) -> Vec<(usize, usize)> {
        let max_x = grid[0].len() - 1;
        let max_y = grid.len() - 1;

        vec![
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
            (if x == max_x { x } else { x + 1 }, y),
            (x, if y == max_y { y } else { y + 1 }),
        ]
    }
}

fn parse_grid(source: &str) -> (Vec<Vec<u16>>, Pos, Pos) {
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    (
        source
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Pos(x, y);
                            0
                        }
                        'E' => {
                            end = Pos(x, y);
                            27
                        }
                        'a'..='z' => (c as u16) - 96,
                        _ => unimplemented!("Unknown char! {c}"),
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "31");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "29");
    }

    #[test]
    fn test_parsing() {
        let i = "SabczE";
        let (r, s, e) = parse_grid(i);
        assert_eq!(r.len(), 1);
        assert_eq!(s, Pos(0, 0));
        assert_eq!(e, Pos(5, 0));
        assert_eq!(r[0].len(), 6);
        assert_eq!(r[0], vec![0, 1, 2, 3, 26, 27]);
    }

    #[test]
    fn test_successors() {
        let i = "abc\nSfd\nqEe";
        let (r, s, e) = parse_grid(i);

        assert_eq!(s, Pos(0, 1));
        assert_eq!(e, Pos(1, 2));

        let sc = s.successors(&r);
        assert_eq!(sc.len(), 1);
    }

    #[test]
    #[ignore] // rules for going to the end require the entire alphabet
    fn test_parsing_2() {
        let i = "abc\nSfd\nqEe";
        let (r, s, e) = parse_grid(i);
        assert_eq!(r.len(), 3);
        assert_eq!(r[0].len(), 3);

        let res = astar(&s, |p| p.successors(&r), |p| p.distance(&e), |p| *p == e);
        assert_eq!(res.unwrap().1, 6);
    }
}
