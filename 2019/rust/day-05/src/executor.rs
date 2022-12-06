use std::collections::HashMap;

mod instructions;
use instructions::*;
mod memory;
use memory::*;

#[allow(unused, dead_code)]

pub type Opcode = u32;
pub type Instruction = fn(&mut Context) -> usize;
pub type Lookup = HashMap<Opcode, Box<Instruction>>;

pub struct Context<'a> {
    memory: &'a mut Memory,
    input: &'a Data,
    output: &'a mut Data,
    pointer: usize,
}

impl Context<'_> {
    fn get_param(&self, offset: usize) -> i32 {
        self.memory.get_param(self.pointer, offset)
    }

    fn get_value(&self, offset: usize) -> i32 {
        self.memory.get_value(self.pointer, offset)
    }

    fn get_input(&self) -> Option<&i32> {
        self.input.first()
    }

    fn set_output(&mut self, o: i32) {
        self.output.push(o);
    }
}

pub struct Executor {
    memory: Memory,
    current_instruction: usize,
    input: Data,
    pub output: Data,
    instructions: Lookup,
}

impl Executor {
    pub fn new(input: &str) -> Self {
        let memory = Memory::from(input);

        Self::from(memory, vec![])
    }

    pub fn with_input(source: &str, input: &str) -> Self {
        let memory = Memory::from(source);
        let input = parse_input(input);

        Self::from(memory, input)
    }

    pub fn from(memory: Memory, input: Data) -> Self {
        let mut e = Executor {
            memory,
            current_instruction: 0,
            input,
            output: Vec::default(),
            instructions: HashMap::default(),
        };

        load_instructions(&mut e);

        e
    }

    pub fn add_instruction(&mut self, opcode: Opcode, ex: Instruction) {
        self.instructions
            .entry(opcode)
            .or_insert_with(|| Box::new(ex));
    }

    pub fn execute(&mut self) {
        println!("Beginning execution");
        while let Some(pointer) = self.process() {
            self.current_instruction = pointer;
        }
    }

    fn process(&mut self) -> Option<usize> {
        let pointer = self.current_instruction;
        debug_assert!(pointer < self.memory.len());

        let opcode = self.memory.get_opcode(pointer);

        if opcode == 99 {
            // check early to avoid complicated match later
            return None;
        }

        let instruction = self.instructions.get(&opcode).expect("Must be valid!");

        let mut context = Context {
            memory: &mut self.memory,
            pointer,
            input: &self.input,
            output: &mut self.output,
        };

        let pointer = (instruction)(&mut context);

        Some(pointer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_executor_runner(data: &str, result: &str) {
        let mut e = Executor::new(data);

        e.execute();

        assert_eq!(e.memory.peek(), &memory::parse_input(result));
    }

    #[test]
    fn test_executor_simple() {
        test_executor_runner("1002,4,3,4,33", "1002,4,3,4,99");
        test_executor_runner("1101,100,-1,4,0", "1101,100,-1,4,99");

        test_executor_runner("10001,0,0,0,99", "20002,0,0,0,99");
        test_executor_runner("10002,3,3,3,99", "10002,3,3,9,99");
        test_executor_runner("10002,4,4,5,99,0", "10002,4,4,5,99,9801");
    }

    #[test]
    fn test_executor_longer() {
        test_executor_runner("10002,3,3,3,10001,1,1,7,99", "10002,3,3,9,10001,1,1,6,99");
    }

    fn test_ex_input(source: &str, input: &str, result: Vec<i32>) {
        let mut e = Executor::with_input(source, input);

        e.execute();

        assert_eq!(e.output, result);
    }

    #[test]
    fn test_io_short() {
        test_ex_input("3,0,4,0,99", "5", vec![5]);

        test_ex_input("3,9,8,9,10,9,4,9,99,-1,8", "8", vec![1]);
        test_ex_input("3,9,8,9,10,9,4,9,99,-1,8", "7", vec![0]);
        test_ex_input("3,3,1108,-1,8,3,4,3,99", "8", vec![1]);
        test_ex_input("3,9,8,9,10,9,4,9,99,-1,8", "7", vec![0]);
    }

    #[test]
    fn test_io() {
        let i = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        // test_ex_input(i, "5", vec![999]);
        test_ex_input(i, "11", vec![1001]);
        test_ex_input(i, "8", vec![1000]);
    }

    #[test]
    fn test_io_long() {
        test_ex_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", "0", vec![0]);
        test_ex_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", "1", vec![1]);

        test_ex_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", "0", vec![0]);
        test_ex_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", "1", vec![1]);
    }
}
