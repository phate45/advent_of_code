use im::HashSet;
use std::collections::HashMap;

pub fn part1(source: &str) -> String {
    source
        .split_whitespace()
        .into_iter()
        .map(Wire::from)
        .map(|w| w.as_set())
        .reduce(|a, b| a.intersection(b))
        .expect("There must be intersections!")
        .iter()
        .map(|t| t.0.abs() + t.1.abs())
        .min()
        .expect("There must be at least one value!")
        .to_string()
}

pub fn part2(source: &str) -> String {
    let mut w = source.split_whitespace().into_iter().map(Wire::from);

    let (wire1, wire2) = (w.next().unwrap(), w.next().unwrap());
    let intersections = wire1.as_set().intersection(wire2.as_set());

    let mut steps_map = HashMap::new();

    intersections
        .iter()
        .map(|point| {
            let w1 = wire1.find(point);
            let w2 = wire2.find(point);
            (point, (w1, w2))
        })
        .for_each(|(point, steps)| {
            if let Some(old_steps) = steps_map.insert(point, steps) {
                if (old_steps.0 + old_steps.1) < (steps.0 + steps.1) {
                    steps_map.insert(point, old_steps);
                }
            }
        });

    debug_assert!(!steps_map.is_empty());
    steps_map
        .values()
        .into_iter()
        .map(|v| v.0 + v.1)
        .min()
        .expect("We must have a value!")
        .to_string()
}

#[derive(Debug)]
struct Wire {
    path: Vec<(i32, i32, u32)>,
    x: i32,
    y: i32,
    steps: u32,
}

impl Wire {
    fn new() -> Self {
        Wire {
            path: vec![],
            x: 0,
            y: 0,
            steps: 0,
        }
    }

    fn from(path: &str) -> Self {
        let mut w = Self::new();

        path.split(',').for_each(|p| w.walk(p));

        w
    }

    fn walk(&mut self, instruction: &str) {
        // assuming instruction here is always direction followed by number
        // ex: D1, R25, U33, L13
        let direction = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<u32>().unwrap();

        for _ in 0..distance {
            match direction {
                'R' => self.x += 1,
                'L' => self.x -= 1,
                'U' => self.y += 1,
                'D' => self.y -= 1,
                _ => unimplemented!("Unknown direction!"),
            };
            self.steps += 1;
            self.path.push((self.x, self.y, self.steps));
        }
    }

    fn as_set(&self) -> HashSet<(i32, i32)> {
        // ignore the steps here, working solution for part 1
        HashSet::from_iter(self.path.clone().iter().map(|(a, b, _)| (*a, *b)))
    }

    fn find(&self, point: &(i32, i32)) -> u32 {
        self.path
            .iter()
            .find_map(|(x, y, s)| {
                if *x == point.0 && *y == point.1 {
                    return Some(*s);
                }
                None
            })
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const TEST_INPUT2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT1), "159");

        assert_eq!(part1(&TEST_INPUT2), "135");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT1), "610");

        assert_eq!(part2(&TEST_INPUT2), "410");
    }

    #[test]
    fn test_wire_walk() {
        let w = Wire::from("R1,U1,L1,D1");
        assert_eq!(w.path.len(), 4);
        assert_eq!(w.path, vec![(1, 0, 1), (1, 1, 2), (0, 1, 3), (0, 0, 4)]);
    }

    #[test]
    fn test_vec_tuple_sorting() {
        let mut v = vec![(1, 2), (3, 4), (0, 1), (9, 9), (5, 1)];
        v.sort();

        assert_eq!(v, vec![(0, 1), (1, 2), (3, 4), (5, 1), (9, 9)])
    }
}
