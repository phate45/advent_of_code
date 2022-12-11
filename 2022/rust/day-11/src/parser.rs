use super::*;
use nom::{
    bytes::complete::{tag, take, take_till, take_until1},
    character::{
        self,
        complete::{one_of, space1},
    },
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn parse_operation(i: &str) -> IResult<&str, Op> {
    let (i, val) = take_until1("\n")(i)?;

    if val == "old * old" {
        return Ok((i, Op::Pow));
    }

    let (_, (op, num)) = preceded(
        tag("old "),
        separated_pair(one_of("+*"), space1, character::complete::u64),
    )(val)?;

    let r = match op {
        '+' => Op::Add(num),
        '*' => Op::Mul(num),
        _ => unimplemented!("Unknown operation! {op:?}"),
    };

    Ok((i, r))
}

pub fn parse_monkey_info(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = pair(take_until1("S"), tag("Starting items: "))(i)?; // skip to the interesting part

    let (i, nums) = take_until1("\n")(i)?;
    let (_, items) = separated_list1(tag(", "), character::complete::u64)(nums)?;

    let (i, _) = pair(take_until1("="), take(2u8))(i)?; // '=' from "new =" and "= " itself

    let (i, op) = parse_operation(i)?;

    let (i, _) = take_till(|c: char| c.is_numeric())(i)?; // skip to the divisible by number
    let (i, test_num) = character::complete::u64(i)?;

    let (i, _) = take_till(|c: char| c.is_numeric())(i)?; // skip to the true clause number
    let (i, true_monke) = character::complete::u32(i)?;

    let (i, _) = take_till(|c: char| c.is_numeric())(i)?; // skip to the false clause number
    let (i, false_monke) = character::complete::u32(i)?;

    // dbg!(&i);

    Ok((
        i,
        Monkey {
            items: VecDeque::from(items),
            op,
            test_num,
            test_true_false: (true_monke as usize, false_monke as usize),
            ..Default::default()
        },
    ))
}

#[cfg(test)]
mod tests {
    use nom::{bytes::complete::take_while, combinator::map_parser};

    use super::*;

    fn test_wrap(i: &str) -> IResult<&str, (char, u64)> {
        preceded(
            tag("old "),
            separated_pair(one_of("+*"), space1, character::complete::u64),
        )(i)
    }

    #[test]
    fn test_parsing_parts() {
        let i = "old * 1";
        let (i, r) = test_wrap(i).unwrap();
        assert_eq!(i, "");
        assert_eq!(r, ('*', 1));
    }

    fn test_wrap2(i: &str) -> IResult<&str, Vec<u32>> {
        // i tried and failed, but there must be a better way
        nom::error::context(
            "How to parse this all at once?",
            separated_list1(
                take_till(|c: char| c.is_numeric()),
                map_parser(
                    take_while(|c: char| c.is_numeric()),
                    character::complete::u32,
                ),
            ),
        )(i)
    }

    #[test]
    #[ignore]
    fn test_parsing_parts2() {
        let i = "  Test: divisible by 19\n    If true: throw to monkey 7\n    If false: throw to monkey 0\n";
        let (i, r) = test_wrap2(i).unwrap();
        assert_eq!(i, "");
        assert_eq!(r, vec![19, 7, 0]);
    }

    #[test]
    fn test_parse_op() {
        let op = "old * old\n";
        let r = parse_operation(op).unwrap().1;
        assert_eq!(r, Op::Pow);

        let op = "old * 3\n";
        let r = parse_operation(op).unwrap().1;
        assert_eq!(r, Op::Mul(3));
    }

    #[test]
    fn test_parse_monkey_info() {
        let binding = std::fs::read_to_string("./test_input.txt").unwrap();
        let single_monkey = binding.split_once("\n\n").unwrap().0;
        // dbg!(&single_monkey);
        let m = parse_monkey_info(single_monkey).unwrap().1;
        assert_eq!(
            m,
            Monkey {
                items: VecDeque::from([79, 98]),
                op: Op::Mul(19),
                test_num: 23,
                test_true_false: (2, 3),
                activity: 0
            }
        );
    }
}
