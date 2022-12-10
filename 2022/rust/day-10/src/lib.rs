use std::collections::BTreeMap;

use color_eyre::Result;
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::separated_pair, IResult};

pub fn part1(source: &str) -> Result<String> {
    let mut scores: BTreeMap<i32, i32> = BTreeMap::new();

    let interesting_cycles = [20, 60, 100, 140, 180, 220];
    process(source, |counter, register| {
        if interesting_cycles.contains(&(counter + 1)) {
            // dbg!(1, &register, &counter, register * (1+counter));
            scores.insert(counter + 1, (counter + 1) * register);
        }

        if interesting_cycles.contains(&(counter + 2)) {
            // dbg!(2, &register, &counter, register * (counter+2));
            scores.insert(counter + 2, (counter + 2) * register);
        }
    });

    Ok(scores.values().sum::<i32>().to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let mut res = "".to_string();

    process(source, |counter, register| {
        if counter % 40 == 0 && counter > 0 {
            res += "\n";
        }

        let sprite_pos = (register - 1)..=(register + 1);
        if sprite_pos.contains(&(counter % 40)) {
            res += "#";
        } else {
            res += ".";
        }
    });

    Ok(res)
}

fn process(input: &str, mut actor: impl FnMut(i32, i32)) {
    let mut counter = 0;
    let mut register = 1;

    for (_cc, curr_line) in input.lines().enumerate() {
        let ins = parse_instruction(curr_line);
        let i;
        if let Ok(ii) = ins {
            i = ii.1;
        } else {
            break;
        }

        // println!("{_cc}: ({counter}) => {:?}", &i);
        actor(counter, register);

        match i {
            Instruction::Noop => {
                counter += 1;
            }
            Instruction::Addx(n) => {
                actor(counter + 1, register);
                counter += 2;
                register += n;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_noop(i: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(i)
}

fn parse_addrx(i: &str) -> IResult<&str, Instruction> {
    map(
        separated_pair(tag("addx"), tag(" "), nom::character::complete::i32),
        |(_, q)| Instruction::Addx(q),
    )(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((parse_addrx, parse_noop))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "13140");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        let output = fs::read_to_string("./test2_output.txt").unwrap();
        let result = part2(&source).unwrap();
        println!("{}", &result);
        println!("{}", &output);
        assert_eq!(result + "\n", output);
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse_instruction("noop").unwrap().1, Instruction::Noop);
        assert_eq!(
            parse_instruction("addx 13").unwrap().1,
            Instruction::Addx(13)
        );
        assert_eq!(
            parse_instruction("addx -1").unwrap().1,
            Instruction::Addx(-1)
        );
    }
}
