use nom::{IResult, multi::{separated_list1}, combinator::all_consuming, character::complete::{newline, alpha1, line_ending}, sequence::{preceded, delimited}, bytes::complete::tag, branch::alt};

use super::*;

pub fn parse(i: &str) -> IResult<&str, Vec<Node>> {
    all_consuming(separated_list1(line_ending, parse_line))(i.trim_end())
}

fn parse_line(i: &str) -> IResult<&str, Node> {
    let (i, name) = delimited(tag("Valve "), alpha1, tag(" has "))(i)?;
    let (i, flow_rate) = delimited(
        tag("flow rate="),
        nom::character::complete::u32,
        tag(";")
    )(i)?;
    let (i, tunnels) = preceded(
        alt((tag(" tunnels lead to valves "), tag(" tunnel leads to valve "))),
        separated_list1(tag(", "), alpha1)
    )(i)?;

    Ok((i,
       Node::new(
           name.to_string(),
           flow_rate,
           tunnels.iter().map(|i| i.to_string()).collect()
       )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let i = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let p = parse_line(i).unwrap().1;
        assert_eq!(p, Node::new("AA".to_string(), 0, vec!["DD".to_string(), "II".to_string(), "BB".to_string()]));

        let i = "Valve HH has flow rate=22; tunnel leads to valve GG";
        let p = parse_line(i).unwrap().1;
        assert_eq!(p, Node::new("HH".to_string(), 22, vec!["GG".to_string()]));
    }
}
