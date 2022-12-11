mod parser;

use color_eyre::Result;
use itertools::Itertools;
use parser::*;
use std::collections::VecDeque;

pub fn part1(source: &str) -> Result<String> {
    let mut monkeys = source
        .split("\n\n")
        .map(|m| parse_monkey_info(m).unwrap().1)
        .collect::<Vec<Monkey>>();

    for (i, m) in monkeys.iter().enumerate() {
        println!("Monkey {i}:\n{m}");
    }

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            // println!("Monkey {i}:");
            let monke = &mut monkeys[i];
            let mut throws = VecDeque::new();

            while let Some(item) = monke.items.pop_front() {
                // println!("  Monkey inspects an item with a worry level of {item}.");
                let worry = monke.inspect(item);
                // println!("    Worry level is changed to {worry}.");
                let worry = worry / 3; // real worry
                // println!("    Monkey gets bored with item. Worry level is changed to {worry}.");
                let test = monke.test(worry);
                // println!("    Current worry level is not divisible by {}.", monke.test_num);
                let target = monke.throw_to(test);
                // println!("    Item with worry level {worry} is thrown to monkey {target}.");
                throws.push_back((target, worry));
            }

            while let Some((t, i)) = throws.pop_front() {
                // println!("  Throwing {i} to Monkey {t}.");
                monkeys[t].items.push_back(i);
            }
        }

        println!("Round {_round}:");
        Monkey::list(&monkeys);
        println!();
    }

    let res = monkeys
        .iter()
        .map(|m| m.activity)
        .sorted()
        .rev()
        .take(2)
        .product::<u64>();

    Ok(res.to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let mut monkeys = source
        .split("\n\n")
        .map(|m| parse_monkey_info(m).unwrap().1)
        .collect::<Vec<Monkey>>();

    // there is no common divisor because all the numbers are prime
    // try with the product of all of them instead
    let c: u64 = monkeys.iter().map(|m| m.test_num).product();

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let monke = &mut monkeys[i];
            let mut throws = VecDeque::new();

            while let Some(item) = monke.items.pop_front() {
                let worry = monke.inspect(item);
                let worry = worry % c; // real worry
                let test = monke.test(worry);
                let target = monke.throw_to(test);
                throws.push_back((target, worry));
            }

            while let Some((t, i)) = throws.pop_front() {
                // println!("  Throwing {i} to Monkey {t}.");
                monkeys[t].items.push_back(i);
            }
        }

        if _round % 1000 == 0 {
            println!("Round {_round}:");
            Monkey::list(&monkeys);
            println!();
        }
    }

    let res = monkeys
        .iter()
        .map(|m| m.activity)
        .sorted()
        .rev()
        .take(2)
        .product::<u64>();

    Ok(res.to_string())
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Op {
    Add(u64),
    Mul(u64),
    #[default]
    Pow,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test_num: u64,
    test_true_false: (usize, usize),
    activity: u64,
}

impl Monkey {
    fn list(monkeys: &[Self]) {
        for (i, m) in monkeys.iter().enumerate() {
            println!("Monkey: {i}: {}", m.items.iter().join(", "));
        }
    }

    fn inspect(&mut self, item: u64) -> u64 {
        self.activity += 1;
        match self.op {
            Op::Pow => item * item,
            Op::Add(n) => item + n,
            Op::Mul(n) => item * n,
        }
    }

    fn test(&self, worry: u64) -> bool {
        worry % self.test_num == 0
    }

    fn throw_to(&self, test: bool) -> usize {
        if test {
            self.test_true_false.0
        } else {
            self.test_true_false.1
        }
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  Starting items: ")?;
        writeln!(f, "{}", self.items.iter().join(", "))?;
        write!(f, "  Operation: new = old ")?;
        match self.op {
            Op::Pow => writeln!(f, "* old")?,
            Op::Add(n) => writeln!(f, "+ {n}")?,
            Op::Mul(n) => writeln!(f, "* {n}")?,
        };
        writeln!(f, "  Test: divisible by {}", self.test_num)?;
        writeln!(f, "    If true: throw to monkey {}", self.test_true_false.0)?;
        writeln!(f, "    If false: throw to monkey {}", self.test_true_false.1)?;

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
        assert_eq!(part1(&source).unwrap(), "10605");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "2713310158");
    }
}
