use std::collections::VecDeque;

pub fn part1(source: &str) -> String {
    let (mut stacks, moves) = parse(source);

    solve(moves, |(times, from, to)| {
        for _ in 0..times {
            let v = stacks[from].pop_front().unwrap();
            stacks[(to)].push_front(v);
        }
    });

    report(&mut stacks)
}

pub fn part2(source: &str) -> String {
    let (mut stacks, moves) = parse(source);

    solve(moves, |(times, from, to)| {
        let mut r = VecDeque::new();
        for _ in 0..times {
            let v = stacks[from].pop_front().unwrap();
            r.push_back(v);
        }
        while let Some(i) = r.pop_back() {
            stacks[(to)].push_front(i);
        }
    });

    report(&mut stacks)
}

fn report(stacks: &mut [VecDeque<char>]) -> String {
    stacks
        .iter_mut()
        .map(|s| s.pop_front().unwrap().to_owned())
        .collect::<String>()
}

fn solve(moves: &str, solver: impl FnMut((usize, usize, usize))) {
    moves.lines().map(parse_move).for_each(solver)
}

fn make_stacks(source: &mut std::str::Split<&str>) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = vec![];

    source
        .next()
        .unwrap()
        .lines() // ex: '[Z] [M] [P]' or '    [D]'
        .for_each(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| ('A'..='Z').contains(c))
                .for_each(|(i, c)| {
                    if (1..).step_by(4).any(|n| n == i) {
                        debug_assert_eq!((i - 1) % 4, 0);

                        let real_i = (i - 1) / 4;

                        while stacks.len() < real_i + 1 {
                            stacks.push(VecDeque::new());
                        }

                        stacks[real_i].push_back(c);
                    }
                })
        });

    stacks
}

fn parse_move(line: &str) -> (usize, usize, usize) {
    let v = line
        .split_whitespace()
        .filter_map(|l| {
            if let Ok(p) = l.parse::<usize>() {
                return Some(p);
            }

            None
        })
        .collect::<Vec<usize>>();

    (v[0], v[1] - 1, v[2] - 1)
}

fn parse(source: &str) -> (Vec<VecDeque<char>>, &str) {
    let mut source_split = source.split("\n\n");
    (make_stacks(&mut source_split), source_split.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source), "CMZ");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source), "MCD");
    }

    #[test]
    fn test_iter_if() {
        assert!((1..).step_by(4).any(|n| n == 1));
        assert!((1..).step_by(4).any(|n| n == 5));
        assert!((1..).step_by(4).any(|n| n == 9));
        assert!((1..).step_by(4).any(|n| n == 33));
    }
}
