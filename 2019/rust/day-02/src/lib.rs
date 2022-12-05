
pub fn part1(source: &str) -> String {
    let mut e = Executor::new(source);

    if e.memory.len() > 20 {
        // test input vs real input .. the latter is much longer and has additional preprocessing
        e.memory[1] = 12;
        e.memory[2] = 2;
    }

    e.execute();

    e.memory.first().unwrap().to_string()
}

pub fn part2(source: &str) -> String {
    use itertools::Itertools;

    let default_memory = parse_input(source);

    let res = (0..=99).cartesian_product(0..=99)
        .into_iter()
        .filter(|(noun, verb)| {
            let mut e = Executor::from(default_memory.clone());
            e.memory[1] = *verb;
            e.memory[2] = *noun;

            e.execute();

            &19690720 == e.memory.first().unwrap()
        })
        .map(|(noun, verb)| {
            100 * verb + noun
        })
        .collect::<Vec<u32>>();

    debug_assert_eq!(res.len(), 1);

    res[0].to_string()
}


struct Executor {
    memory: Vec<u32>,
    current_instruction: u32,
}

impl Executor {
    fn new(input: &str) -> Self {
        let memory = parse_input(input);

        Self::from(memory)
    }

    fn from(memory: Vec<u32>) -> Self {
        Executor {
            memory,
            current_instruction: 0,
        }
    }

    fn execute(&mut self) {
        while self.process() {
            self.current_instruction += 4;
        }
    }

    #[allow(unused)]  // i'm sure it will be used in a later day
    fn step_over(&mut self) {
        self.process();
        self.current_instruction += 4;
    }

    fn process(&mut self) -> bool {
        let index = self.current_instruction as usize;
        debug_assert!(index < self.memory.len());

        let instruction = self.memory[index];

        if instruction == 99 {
            // check early to avoid complicated match later
            return false;
        }

        let (val1, val2, target) = self.get_data(&self.current_instruction);

        let res: u32 = match instruction {
            1 => val1 + val2, // add
            2 => val1 * val2, // multiply
            _ => unimplemented!("Invalid instruction!"),
        };

        self.memory[target as usize] = res;

        true
    }

    fn get_data(&self, index: &u32) -> (&u32, &u32, u32) {
        (
            self.memory.get(self.memory.get((index + 1) as usize).unwrap().to_owned() as usize).unwrap(),
            self.memory.get(self.memory.get((index + 2) as usize).unwrap().to_owned() as usize).unwrap(),
            self.memory.get((index + 3) as usize).unwrap().to_owned(),
        )
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .into_iter()
        .map(|c| c.parse::<u32>().unwrap())
        .collect()
}

// fn diff(v1: Vec<u32>, v2: Vec<u32>) {
//     debug_assert_eq!(v1.len(), v2.len());
// }

// #[macro_export]
// macro_rules! uniq {
//     ( $v:expr ) => {{
//         let mut out = $crate::uniq!();
//         for i in $v {
//             out.insert(i);
//         }
//         out
//     }};
//     ( $i1:expr, $i2:expr, $i3:expr) => {{

//     }};
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source), "3500");
    }

    #[test]
    fn test_part2() {
        // there is no example for the part 2 to test
        assert!(true);
    }

    #[test]
    fn test_input_parser() {
        let r = parse_input("1,2,3,4");
        assert_eq!(r, vec![1, 2, 3, 4]);
    }

    fn test_executor_runner(data: &str, result: &str) {
        let mut e = Executor::new(data);
        e.execute();

        assert_eq!(e.memory, parse_input(result));
    }

    #[test]
    fn test_executor_simple() {
        test_executor_runner("1,0,0,0,99", "2,0,0,0,99");
        test_executor_runner("2,3,0,3,99", "2,3,0,6,99");
        test_executor_runner("2,4,4,5,99,0", "2,4,4,5,99,9801");
        test_executor_runner("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")
    }
}
