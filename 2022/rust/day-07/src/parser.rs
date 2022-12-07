use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Entry(Entry),
}

pub fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        use std::fs;

        let source = fs::read_to_string("./test_input.txt").unwrap();
        for l in source.lines() {
            let r = parse_line(l);
            assert!(r.is_ok());
            println!("{l}");
            println!("{:?}", r.unwrap().1);
        }
    }
}
