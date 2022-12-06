use super::{Context, Executor};
use crate::get2;
use std::io::Write;

pub type Data = Vec<i32>;

pub fn load_instructions(executor: &mut Executor) {
    executor.add_instruction(1, ins_add);
    executor.add_instruction(2, ins_multiply);
    executor.add_instruction(3, ins_input);
    executor.add_instruction(4, ins_output);
    executor.add_instruction(5, ins_jump_true);
    executor.add_instruction(6, ins_jump_false);
    executor.add_instruction(7, ins_less_than);
    executor.add_instruction(8, ins_equals);
}

fn ins_add(ctx: &mut Context) -> usize {
    let (v1, v2) = get2!(ctx);
    let target = ctx.get_value(3);

    ctx.memory.set(target as usize, v1 + v2);

    ctx.pointer + 4
}

fn ins_multiply(ctx: &mut Context) -> usize {
    let (v1, v2) = get2!(ctx);
    let target = ctx.get_value(3);

    ctx.memory.set(target as usize, v1 * v2);

    ctx.pointer + 4
}

fn ins_input(ctx: &mut Context) -> usize {
    use std::io;

    let v = if let Some(i) = ctx.get_input() {
        println!("Loaded input: {i}");
        _ = io::stdout().flush();
        *i
    } else {
        let mut input = String::new();

        print!("Input instruction: ");
        _ = io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    };

    let target = ctx.get_value(1);

    ctx.memory.set(target as usize, v);

    ctx.pointer + 2
}

fn ins_output(ctx: &mut Context) -> usize {
    let target = ctx.get_value(1);
    let v = ctx.memory.get_value(0, target as usize);
    println!("Output: {v}");
    ctx.set_output(v);

    ctx.pointer + 2
}

fn ins_jump_true(ctx: &mut Context) -> usize {
    let v1 = ctx.get_param(1);

    if v1 != 0 {
        let v2 = ctx.get_param(2);
        return v2 as usize;
    }

    ctx.pointer + 3
}

fn ins_jump_false(ctx: &mut Context) -> usize {
    let v1 = ctx.get_param(1);

    if v1 == 0 {
        let v2 = ctx.get_param(2);
        return v2 as usize;
    }

    ctx.pointer + 3
}

fn ins_less_than(ctx: &mut Context) -> usize {
    let (v1, v2) = get2!(ctx);
    let target = ctx.get_value(3);

    ctx.memory.set(target as usize, i32::from(v1 < v2));

    ctx.pointer + 4
}

fn ins_equals(ctx: &mut Context) -> usize {
    let (v1, v2) = get2!(ctx);
    let target = ctx.get_value(3);

    ctx.memory.set(target as usize, i32::from(v1 == v2));

    ctx.pointer + 4
}

#[macro_export]
macro_rules! get2 {
    ( $c:expr ) => {{
        let a = $c.get_param(1);
        let b = $c.get_param(2);
        (a, b)
    }};
}
