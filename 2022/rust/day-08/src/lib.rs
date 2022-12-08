use color_eyre::Result;
use itertools::rev;

pub fn part1(source: &str) -> Result<String> {
    let data = Data(parse_input(source));

    let mut r: u32 = 0;

    for row in 0..data.len() {
        for column in 0..data.len() {
            if data.is_visible(row, column) {
                r += 1;
            }
        }
    }

    Ok(r.to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let data = Data(parse_input(source));

    let res = data
        .0
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, column)| data.get_scenic_score(column, x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    Ok(res.to_string())
}

fn parse_input(source: &str) -> Vec<Vec<u8>> {
    source
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect()
}

struct Data(Vec<Vec<u8>>);
impl Data {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_scenic_score(&self, curr_value: &u8, x: usize, y: usize) -> u32 {
        let mut r_top = 0;
        for i in rev(0..x) {
            r_top += 1;
            if self.0[i][y] >= *curr_value {
                break;
            }
        }

        let mut r_bottom = 0;
        for i in (x + 1)..self.len() {
            r_bottom += 1;
            if self.0[i][y] >= *curr_value {
                break;
            }
        }

        let mut r_left = 0;
        for i in rev(0..y) {
            r_left += 1;
            if self.0[x][i] >= *curr_value {
                break;
            }
        }

        let mut r_right = 0;
        for i in (y + 1)..self.len() {
            r_right += 1;
            if self.0[x][i] >= *curr_value {
                break;
            }
        }

        r_top * r_bottom * r_right * r_left
    }

    fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.len() - 1 || y == self.len() - 1
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.is_edge(x, y) || !self.check_sides(x, y)
    }

    fn check_sides(&self, x: usize, y: usize) -> bool {
        let curr = self.0[x][y];

        let mut taller_top = false;
        for i in 0..x {
            if self.0[i][y] >= curr {
                taller_top = true;
            }
        }

        let mut taller_bottom = false;
        for i in (x + 1)..self.len() {
            if self.0[i][y] >= curr {
                taller_bottom = true;
            }
        }

        let mut taller_left = false;
        for i in 0..y {
            if self.0[x][i] >= curr {
                taller_left = true;
            }
        }

        let mut taller_right = false;
        for i in (y + 1)..self.len() {
            if self.0[x][i] >= curr {
                taller_right = true;
            }
        }

        taller_top && taller_bottom && taller_left && taller_right
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "21");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "8");
    }

    #[test]
    fn test_data_sides() {
        let d = Data(parse_input("123\n456\n789"));
        assert!(d.is_visible(1, 1));
    }

    #[test]
    fn test_data_edge() {
        let d = Data(parse_input("123\n456\n789"));
        assert_eq!(d.len(), 3);
        assert!(d.is_edge(0, 0));
        assert!(d.is_edge(0, 1));
        assert!(d.is_edge(0, 2));
        assert!(d.is_edge(0, 0));
        assert!(d.is_edge(1, 0));
        assert!(d.is_edge(2, 0));
        assert!(!d.is_edge(1, 1));
        assert!(d.is_edge(2, 1));
        assert!(d.is_edge(2, 2));
        assert!(d.is_edge(1, 2));
    }

    #[test]
    fn test_scenic_score() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        let d = Data(parse_input(&source));
        assert_eq!(d.get_scenic_score(&5, 1, 2), 4);
        assert_eq!(d.get_scenic_score(&5, 3, 2), 8);
    }

    #[test]
    fn test_parse_input() {
        let i = "123";
        assert_eq!(parse_input(i), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_rev_iter() {
        let mut q = rev(0..3);
        assert_eq!(q.next().unwrap(), 2);
    }
}
