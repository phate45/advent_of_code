use ::int_enum::IntEnum;
use std::cmp::Ordering;

#[repr(u16)]
#[derive(Debug, Copy, Clone, IntEnum)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Game(Hand, Hand);

impl Hand {
    fn from(source: &str) -> Self {
        use Hand::*;

        debug_assert_eq!(source.len(), 1);

        let hand: &str = &source.chars().next().unwrap().to_string();
        match hand {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unimplemented!("Unknown hand!"),
        }
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.int_value().eq(&other.int_value())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use Hand::*;

        match (*self, *other) {
            (Rock, Scissors) => return Greater,
            (Scissors, Rock) => return Less,
            _ => (),
        }

        self.int_value().cmp(&other.int_value())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    fn resolve(&self) -> u16 {
        use Ordering::*;

        let hand_value = self.1.int_value();
        match self.1.cmp(&self.0) {
            Equal => hand_value + 3,
            Greater => hand_value + 6,
            Less => hand_value,
        }
    }

    fn resolve_2(&self) -> u16 {
        use Hand::*;

        match self.1 {
            Rock => {
                // lose
                match self.0 {
                    Rock => Game(Rock, Scissors).resolve(),
                    Paper => Game(Paper, Rock).resolve(),
                    Scissors => Game(Scissors, Paper).resolve(),
                }
            },
            Paper => {
                // draw
                Game(self.0, self.0).resolve()
            },
            Scissors => {
                // win
                match self.0 {
                    Rock => Game(Rock, Paper).resolve(),
                    Paper => Game(Paper, Scissors).resolve(),
                    Scissors => Game(Scissors, Rock).resolve(),
                }
            }
        }
    }
}

pub fn part1(source: &str) -> String {
    parse(source, |g: Game| g.resolve().into())
}

pub fn part2(source: &str) -> String {
    parse(source, |g: Game| g.resolve_2().into())
}

fn parse(source: &str, resolver: impl Fn(Game) -> u32) -> String {
    source.lines()
        .map(|l| {
            let hands = l.split_whitespace()
                .map(Hand::from)
                .collect::<Vec<Hand>>();
            Game(*hands.first().unwrap(), *hands.last().unwrap())
        })
        .map(resolver)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Hand::*;
    use std::fs;

    #[test]
    fn test_hand_creation() {
        assert_eq!(Hand::from("A"), Rock);
        assert_eq!(Hand::from("X"), Rock);
        assert_eq!(Hand::from("B"), Paper);
        assert_eq!(Hand::from("Y"), Paper);
        assert_eq!(Hand::from("C"), Scissors);
        assert_eq!(Hand::from("Z"), Scissors);
    }

    #[test]
    fn test_hand_cmp() {
        assert!(Rock == Rock);
        assert!(Paper == Paper);
        assert!(Scissors == Scissors);

        assert!(Paper > Rock);
        assert!(Scissors > Paper);
        assert!(Rock > Scissors);

        assert!(Rock < Paper);
        assert!(Paper < Scissors);
        assert!(Scissors < Rock);

        assert_eq!(Rock.int_value(), 1);
    }

    #[test]
    fn test_game_resolution() {
        // draw = value of hand + 3
        let game = Game(Rock, Rock);
        assert_eq!(game.resolve(), 4);

        let game = Game(Paper, Paper);
        assert_eq!(game.resolve(), 5);

        let game = Game(Scissors, Scissors);
        assert_eq!(game.resolve(), 6);

        // loss = value of hand
        let game = Game(Paper, Rock);
        assert_eq!(game.resolve(), 1);

        let game = Game(Scissors, Paper);
        assert_eq!(game.resolve(), 2);

        let game = Game(Rock, Scissors);
        assert_eq!(game.resolve(), 3);

        // win = value of hand + 6
        let game = Game(Rock, Paper);
        assert_eq!(game.resolve(), 8);

        let game = Game(Paper, Scissors);
        assert_eq!(game.resolve(), 9);

        let game = Game(Scissors, Rock);
        assert_eq!(game.resolve(), 7);
    }

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source), "15");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source), "12");
    }
}
