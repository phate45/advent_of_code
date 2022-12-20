use std::{num::ParseIntError, str::FromStr};

use color_eyre::Result;

pub fn part1(source: &str) -> Result<String> {
    let mut vals = parse(source);
    sort_all(&mut vals);

    Ok(get_result(&vals).to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let mut vals: Vec<Value> = parse(source)
        .iter()
        .map(|vl| Value {
            v: vl.v * 811589153,
            index: vl.index,
        })
        .collect();

    for _i in 0..10 {
        // dbg!(&_i);
        sort_all(&mut vals);
    }

    Ok(get_result(&vals).to_string())
}

fn get_result(vals: &[Value]) -> i64 {
    let (ix, _) = vals
        .iter()
        .copied()
        .enumerate()
        .find(|(_, v)| v.v == 0)
        .expect("The value must exist!");

    let x1 = git(1000, ix, vals);
    let x2 = git(2000, ix, vals);
    let x3 = git(3000, ix, vals);

    vals[x1].v + vals[x2].v + vals[x3].v
}

fn git(n: usize, ix: usize, vals: &[Value]) -> usize {
    (n + ix) % vals.len()
}

fn parse(source: &str) -> Vec<Value> {
    source
        .lines()
        .enumerate()
        .map(|l| {
            let mut v = l.1.parse::<Value>().expect("Must be valid!");
            v.index = l.0;
            v
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Value {
    v: i64,
    index: usize,
}

impl FromStr for Value {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            v: s.parse().expect("Must be a valid i64!"),
            index: 0,
        })
    }
}

fn sort_all(vals: &mut Vec<Value>) {
    for i in 0..vals.len() {
        sort_one(vals, i);
    }
}

fn sort_one(vals: &mut Vec<Value>, index: usize) {
    let (ix, curr) = vals
        .iter()
        .copied()
        .enumerate()
        .find(|(_, v)| v.index == index)
        .expect("The value must exist!");

    if curr.v == 0 {
        return;
    }

    let current = vals.remove(ix);
    let added = ix as i64 + current.v;
    let nx = added.rem_euclid(vals.len() as i64);

    vals.insert(nx as usize, current);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "3");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "1623178306");
    }

    fn pp(vals: &[Value]) {
        println!(
            "{}",
            vals.iter()
                .map(|val| val.v.to_string() + ", ")
                // .intersperse_with(", ")
                .collect::<String>()
        )
    }

    #[test]
    fn test_repeat_sort() {
        let mut vals: Vec<Value> = std_vals()
            .iter()
            .map(|vl| Value {
                v: vl.v * 811589153,
                index: vl.index,
            })
            .collect();

        pp(&vals);
        sort_all(&mut vals);
        pp(&vals);
        // this was just to visually verify the sort for part 2
        // assert!(false);
    }

    #[test]
    fn test_rotating() {
        let mut s = vec![1, 2, 3];
        s.rotate_left(1);
        assert_eq!(s, vec![2, 3, 1]);
        s.rotate_left(1);
        assert_eq!(s, vec![3, 1, 2]);
    }

    fn std_vals() -> Vec<Value> {
        vec![1, 2, -3, 3, -2, 0, 4]
            .iter()
            .enumerate()
            .map(|(i, &v)| Value { v, index: i })
            .collect()
    }

    #[test]
    fn test_parse() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        let vals: Vec<Value> = std_vals();
        assert_eq!(parse(&source), vals)
    }

    #[test]
    fn test_swap_one() {
        let mut s = parse("1\n2\n3\n");
        let r = vec![
            Value { v: 2, index: 1 },
            Value { v: 1, index: 0 },
            Value { v: 3, index: 2 },
        ];
        sort_one(&mut s, 0);
        assert_eq!(s, r);
    }

    #[test]
    fn test_wrap_negative() {
        let mut s = parse("4\n-2\n5\n6");
        let r = vec![
            Value { v: 4, index: 0 },
            Value { v: 5, index: 2 },
            Value { v: -2, index: 1 },
            Value { v: 6, index: 3 },
        ];
        sort_one(&mut s, 1);
        assert_eq!(s, r);
    }

    #[test]
    fn test_wrap_neg_2() {
        let mut s = parse("4\n5\n-2\n6");
        let r = vec![
            Value { v: -2, index: 2 },
            Value { v: 4, index: 0 },
            Value { v: 5, index: 1 },
            Value { v: 6, index: 3 },
        ];
        sort_one(&mut s, 2);
        assert_eq!(s, r);
    }

    #[test]
    fn test_swap_boundary() {
        let mut s = parse("2\n3\n1\n");
        let r = vec![
            Value { v: 2, index: 0 },
            Value { v: 1, index: 2 },
            Value { v: 3, index: 1 },
        ];
        sort_one(&mut s, 2);
        assert_eq!(s, r);
    }

    #[test]
    fn test_swap_zero() {
        let mut s = parse("1\n0\n3\n");
        let r = vec![
            Value { v: 1, index: 0 },
            Value { v: 0, index: 1 },
            Value { v: 3, index: 2 },
        ];
        sort_one(&mut s, 1);
        assert_eq!(s, r);
    }

    #[test]
    fn test_sort_all() {
        let mut s = std_vals();
        sort_all(&mut s);
        let r = vec![
            Value { v: -2, index: 4 },
            Value { v: 1, index: 0 },
            Value { v: 2, index: 1 },
            Value { v: -3, index: 2 },
            Value { v: 4, index: 6 },
            Value { v: 0, index: 5 },
            Value { v: 3, index: 3 },
        ];
        assert_eq!(s, r);
    }

    #[test]
    fn test_find_nth_val() {
        let mut s = std_vals();
        sort_all(&mut s);
        let (ix, _) = s
            .iter()
            .copied()
            .enumerate()
            .find(|(_, v)| v.v == 0)
            .expect("The value must exist!");

        assert_eq!(ix, 5);
        let x1 = git(1000, ix, &s);
        assert_eq!(s[x1].v, 4);
        let x1 = git(2000, ix, &s);
        assert_eq!(s[x1].v, -3);
        let x1 = git(3000, ix, &s);
        assert_eq!(s[x1].v, 2);
    }
}
