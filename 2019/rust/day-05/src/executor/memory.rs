use super::{Data, Opcode};

pub type Loading = Vec<ParameterMode>;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Copy, Clone)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    // fn from(value: usize) -> Self {
    //     Self::from_char(char::from_u32(value as u32).unwrap())
    // }

    fn from_char(value: char) -> Self {
        use ParameterMode::*;

        match value {
            '0' => Position,
            '1' => Immediate,
            _ => unreachable!("Unimaginable! {}", value),
        }
    }
}

pub struct Memory {
    data: Data,
}

impl Memory {
    pub fn from(data: &str) -> Self {
        Memory {
            data: parse_input(data),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_opcode(&self, index: usize) -> Opcode {
        (self.data[index] as u32) % 100
    }

    pub fn get_param_mode(&self, index: usize, offset: usize) -> ParameterMode {
        let mut n = (self.data[index] as usize).to_string();

        while n.len() < 5 {
            // pad the left up to 5
            n = "0".to_owned() + &n;
        }

        let mut loading: Loading = n.chars().take(3).map(ParameterMode::from_char).collect();
        loading.reverse();

        loading[offset.saturating_sub(1)]
    }

    pub fn get_param(&self, index: usize, offset: usize) -> i32 {
        use ParameterMode::*;

        let memory = &self.data;

        let param_mode = self.get_param_mode(index, offset);
        let offset = index + offset;

        match param_mode {
            Position => {
                let address: usize = memory[offset] as usize;
                memory[address]
            }
            Immediate => memory[offset],
        }
    }

    pub fn get_value(&self, index: usize, offset: usize) -> i32 {
        self.data[index + offset]
    }

    pub fn set(&mut self, index: usize, data: i32) {
        self.data[index] = data;
    }

    #[allow(dead_code)] // used for test
    pub fn peek(&self) -> &Data {
        &self.data
    }
}

pub fn parse_input(input: &str) -> Data {
    input
        .trim()
        .split(',')
        .into_iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_loading() {
        let m = Memory::from("10001,0,0,0,99");

        let i = m.get_opcode(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn test_input_parser() {
        let r = parse_input("1,2,3,4");
        assert_eq!(r, vec![1, 2, 3, 4]);
        let r = parse_input("1,-2,3,-4");
        assert_eq!(r, vec![1, -2, 3, -4]);
    }

    #[test]
    fn test_memory_param() {
        let a = Memory::from("1002,4,3,4,33");
        assert_eq!(a.get_opcode(0), 2);
        assert_eq!(a.get_param(0, 1), 33);
        assert_eq!(a.get_param(0, 2), 3);
        assert_eq!(a.get_value(0, 3), 4);
    }

    #[test]
    fn test_opcode_parser() {
        let a = Memory::from("2");
        assert_eq!(a.get_opcode(0), 2);
        let a = Memory::from("20");
        assert_eq!(a.get_opcode(0), 20);
        let a = Memory::from("1020");
        assert_eq!(a.get_opcode(0), 20);
        let a = Memory::from("1122");
        assert_eq!(a.get_opcode(0), 22);
    }
}
