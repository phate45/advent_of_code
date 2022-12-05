pub fn part1(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split_to_tuples(','))
        .map(|(left, right)| (Area::from(left), Area::from(right)))
        .filter(|(left, right)| left.either_contains(right))
        .count()
        .to_string()
}

pub fn part2(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split_to_tuples(','))
        .map(|(left, right)| (Area::from(left), Area::from(right)))
        .filter(|(left, right)| left.overlap(right))
        .count()
        .to_string()
}

struct Area {
    left: u32,
    right: u32,
}

impl Area {
    fn from(assignment: &str) -> Self {
        let (from, to) = assignment.split_to_tuples('-');

        Area {
            left: from.parse::<u32>().unwrap(),
            right: to.parse::<u32>().unwrap(),
        }
    }

    fn overlap(&self, other: &Self) -> bool {
        self.either_contains(other)
            || !(self.right < other.right && self.left < other.left && self.right < other.left
                || other.right < self.right && other.left < self.left && other.right < self.left)
    }

    fn either_contains(&self, other: &Self) -> bool {
        self.same_or_larger(other) || other.same_or_larger(self)
    }

    fn same_or_larger(&self, other: &Self) -> bool {
        self.left <= other.left && self.right >= other.right
    }
}

trait Splitter {
    fn split_to_tuples(&self, at: char) -> (&str, &str);
}

impl Splitter for str {
    fn split_to_tuples(&self, at: char) -> (&str, &str) {
        let mut split = self.split(at);
        (split.next().unwrap(), split.next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source), "2");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source), "4");
    }

    #[test]
    fn test_area_size() {
        let a = Area::from("2-5");
        let b = Area::from("3-5");
        assert!(a.same_or_larger(&b));

        let a = Area::from("1-3");
        let b = Area::from("4-7");
        assert!(!a.same_or_larger(&b));
    }

    #[test]
    fn test_area_contains() {
        let a = Area::from("2-5");
        let b = Area::from("3-5");
        assert!(a.either_contains(&b));

        let a = Area::from("4-6");
        let b = Area::from("6-6");
        assert!(a.either_contains(&b));

        let a = Area::from("1-3");
        let b = Area::from("4-7");
        assert!(!a.either_contains(&b));
    }

    #[test]
    fn test_area_overlap() {
        let a = Area::from("2-4");
        let b = Area::from("6-8");
        assert!(!a.overlap(&b));

        let a = Area::from("2-5");
        let b = Area::from("3-5");
        assert!(a.overlap(&b));
    }

    #[test]
    fn test_splitter() {
        let s = "a,b";
        assert_eq!(s.split_to_tuples(','), ("a", "b"));

        let s = "a-b";
        assert_eq!(s.split_to_tuples('-'), ("a", "b"));
    }
}
