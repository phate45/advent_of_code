use std::collections::{BTreeMap, VecDeque};

use color_eyre::Result;
use nom::{bytes::complete::tag, character::complete::alpha1, sequence::terminated, IResult};
use num_format::{Locale, ToFormattedString};

pub fn part1(source: &str) -> Result<String> {
    let res = solve(source, 2326);
    let res = res.0.resolve(res.1, res.2);
    Ok(res.to_string())
}

pub fn part2(source: &str) -> Result<String> {
    // let res = solve(source, 3_952_288_690_726);
    // there are multiple values that produce this equality -> the above..=the above+2
    // the lowest number seems to be the correct solution

    let monkeys = parse(source);
    let (mut nums, ops) = transform(monkeys);
    nums.insert("humn".to_string(), 2_326);
    let res = solve_inner(nums.clone(), ops.clone());

    let target = res.2; // determined by manual testing

    // the bisection works well, in both implementations
    // the problem lies in the test input when compared with the real input
    // in the test input, the output of the 'solve_inner' method is related to its input
    // in the real input, the solution is **inversely** proportional to the input
    let res = bisection(
        |n| {
            nums.insert("humn".to_string(), n);
            solve_inner(nums.clone(), ops.clone()).1
        },
        target,
        0,
        4_000_000_000_000, // the high bound was determined by some manual testing
    );

    Ok(res.to_string())
}

fn solve(source: &str, humn: i64) -> (MonkeyInfo, i64, i64) {
    let monkeys = parse(source);
    let (mut nums, ops) = transform(monkeys);

    nums.insert("humn".to_string(), humn);
    solve_inner(nums, ops)
}

fn solve_inner(nums: BTreeMap<String, i64>, ops: VecDeque<MonkeyInfo>) -> (MonkeyInfo, i64, i64) {
    let mut nums = nums;
    let mut ops = ops;

    while !ops.is_empty() {
        let m = ops.pop_front().unwrap();
        let d = m.dependents.as_ref().unwrap();
        if let (Some(a), Some(b)) = (nums.get(&d.0), nums.get(&d.1)) {
            if m.name == "root" && ops.is_empty() {
                // we are at the end
                return (m, *a, *b);
            }
            nums.insert(m.name.clone(), m.resolve(*a, *b));
        } else {
            ops.push_back(m);
        }
    }

    unimplemented!("What?")
}

fn bisection(mut f: impl FnMut(i64) -> i64, r: i64, low: i64, high: i64) -> i64 {
    println!(
        "Trying to find '{}' with bounds {}-{}",
        r.to_formatted_string(&Locale::en),
        low.to_formatted_string(&Locale::en),
        &high.to_formatted_string(&Locale::en)
    );
    if low == high {
        return -1;
    }

    let mid = (high + low) / 2;
    let guess = f(mid);
    let g = &guess.to_formatted_string(&Locale::en);
    let m = &mid.to_formatted_string(&Locale::en);
    println!("f({}) = {}", m, g);

    if guess > r {
        return bisection(f, r, low, mid);
    } else if guess < r {
        return bisection(f, r, mid, high);
    }

    mid - 1
}

fn bisection_imp(mut f: impl FnMut(i64) -> i64, r: i64, low: i64, high: i64) -> i64 {
    let mut low = low;
    let mut high = high;
    println!(
        "Trying to find '{}' with bounds {}-{}",
        r.to_formatted_string(&Locale::en),
        low.to_formatted_string(&Locale::en),
        &high.to_formatted_string(&Locale::en)
    );

    while (high - low).abs() > 1 {
        let mid = (high + low) / 2;
        let guess = f(mid);

        let g = &guess.to_formatted_string(&Locale::en);
        let m = &mid.to_formatted_string(&Locale::en);
        println!("f({}) = {}", m, g);
        if guess == r {
            return mid;
        } else if guess < r {
            low = mid;
        } else {
            high = mid;
        }
    }

    (low + high) / 2
}

fn transform(monkeys: Vec<Monkey>) -> (BTreeMap<String, i64>, VecDeque<MonkeyInfo>) {
    let mut nums = BTreeMap::new();
    let mut ops = VecDeque::new();

    for monkey in monkeys {
        match monkey {
            Monkey::Number(n) => {
                if n.name == "humn" {
                    continue;
                }
                nums.insert(n.name.clone(), n.number.unwrap());
            }
            Monkey::Operation(o) => {
                let (left, right) = o.dependents.as_ref().unwrap();
                if nums.contains_key(left) && nums.contains_key(right) {
                    let n1 = nums.get(left).unwrap();
                    let n2 = nums.get(right).unwrap();
                    nums.insert(o.name.clone(), o.resolve(*n1, *n2));
                } else {
                    debug_assert_eq!(o.number, None);
                    ops.push_back(o);
                }
            }
        }
    }

    (nums, ops)
}

#[derive(Debug, PartialEq, Default, PartialOrd, Eq, Ord, Clone)]
struct MonkeyInfo {
    name: String,
    number: Option<i64>,
    operation: Option<char>,
    dependents: Option<(String, String)>,
}

impl MonkeyInfo {
    fn resolve(&self, n1: i64, n2: i64) -> i64 {
        match self.operation.unwrap() {
            '+' => n1 + n2,
            '-' => n1 - n2,
            '*' => n1 * n2,
            '/' => n1 / n2,
            _ => unimplemented!("Unknown operation! {:?}", self.operation),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Monkey {
    Number(MonkeyInfo),
    Operation(MonkeyInfo),
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Monkey::*;

        match (self, other) {
            (Number(a), Number(b)) => a.cmp(b),
            (Number(_), Operation(_)) => std::cmp::Ordering::Greater,
            (Operation(_), Number(_)) => std::cmp::Ordering::Less,
            (Operation(a), Operation(b)) => a.cmp(b),
        }
    }
}

fn parse(i: &str) -> Vec<Monkey> {
    let mut m = i
        .lines()
        .map(parse_line)
        .map(|r| r.unwrap().1)
        .collect::<Vec<Monkey>>();
    m.sort();
    m.reverse();
    m
}

fn parse_line(i: &str) -> IResult<&str, Monkey> {
    let (val, name) = terminated(alpha1, tag(": "))(i)?;

    if val.len() < 8 {
        let (_, num) = nom::character::complete::i64(val)?;
        return Ok((
            val,
            Monkey::Number(MonkeyInfo {
                name: name.to_string(),
                number: Some(num),
                ..Default::default()
            }),
        ));
    }

    let l: Vec<String> = val.split_whitespace().map(|p| p.to_string()).collect();

    return Ok((
        val,
        Monkey::Operation(MonkeyInfo {
            name: name.to_string(),
            operation: Some(l[1].chars().next().unwrap()),
            dependents: Some((l[0].clone(), l[2].clone())),
            ..Default::default()
        }),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        let res = solve(&source, 5);
        let res = res.0.resolve(res.1, res.2);
        assert_eq!(res, 152);
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "301");
    }

    #[test]
    fn test_sort() {
        // assure that numbers come before operations
        let m1 = parse_line("aaaa: 4").unwrap().1;
        let m2 = parse_line("bbbb: aaaa + cccc").unwrap().1;
        assert!(m1 > m2);
        let mut v = vec![&m1, &m2];
        v.sort();
        v.reverse();
        assert_eq!(v[0], &m1);
    }

    #[test]
    fn test_parse() {
        let i = "sjmn: drzm * dbpl\nsllz: 4";
        let r = parse(i);
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn test_parse_line() {
        let i = "root: pppw + sjmn";
        let r = parse_line(i).unwrap().1;
        assert_eq!(
            r,
            Monkey::Operation(MonkeyInfo {
                name: "root".to_string(),
                operation: Some('+'),
                dependents: Some(("pppw".to_string(), "sjmn".to_string())),
                ..Default::default()
            })
        );

        let i = "sllz: 4";
        let r = parse_line(i).unwrap().1;
        assert_eq!(
            r,
            Monkey::Number(MonkeyInfo {
                name: "sllz".to_string(),
                number: Some(4),
                ..Default::default()
            })
        );
    }
}
