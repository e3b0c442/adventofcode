use simple_error::{bail, require_with};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day23(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 23: Opening the Turing Lock");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Op<'a> {
    mnem: &'a str,
    reg: &'a str,
    offset: i32,
}

#[derive(Debug)]
struct State {
    ip: usize,
    a: usize,
    b: usize,
}

fn exec(op: Op, state: &mut State) {
    match op.mnem {
        "hlf" => {
            match op.reg {
                "a" => state.a /= 2,
                "b" => state.b /= 2,
                _ => (),
            };
            state.ip += 1;
        }
        "tpl" => {
            match op.reg {
                "a" => state.a *= 3,
                "b" => state.b *= 3,
                _ => (),
            };
            state.ip += 1;
        }
        "inc" => {
            match op.reg {
                "a" => state.a += 1,
                "b" => state.b += 1,
                _ => (),
            }
            state.ip += 1;
        }
        "jmp" => state.ip = ((state.ip as i32) + op.offset) as usize,
        "jie" => {
            let check = match op.reg {
                "a" => state.a,
                "b" => state.b,
                _ => 0,
            };
            if check % 2 == 0 {
                state.ip = ((state.ip as i32) + op.offset) as usize
            } else {
                state.ip += 1;
            }
        }
        "jio" => {
            let check = match op.reg {
                "a" => state.a,
                "b" => state.b,
                _ => 0,
            };
            if check == 1 {
                state.ip = ((state.ip as i32) + op.offset) as usize
            } else {
                state.ip += 1;
            }
        }
        _ => (),
    }
}

fn input_to_program(input: &str) -> Result<Vec<Op>, Box<dyn Error>> {
    let mut prog = Vec::new();
    for line in input.lines() {
        let parts = line.splitn(2, " ").collect::<Vec<&str>>();
        let (op, data) = (
            *require_with!(parts.get(0), &format!("Invalid input: {}", line)),
            *require_with!(parts.get(1), &format!("Invalid input: {}", line)),
        );
        match op {
            "hlf" | "tpl" | "inc" => {
                prog.push(Op {
                    mnem: op,
                    reg: data,
                    offset: 0,
                });
            }
            "jmp" => {
                let offset = data.parse::<i32>()?;
                prog.push(Op {
                    mnem: op,
                    reg: "",
                    offset: offset,
                });
            }
            "jio" | "jie" => {
                let data = data.split(", ").collect::<Vec<&str>>();
                let (reg, offset) = (
                    *require_with!(data.get(0), &format!("Invalid input: {}", line)),
                    require_with!(data.get(1), &format!("Invalid input: {}", line))
                        .parse::<i32>()?,
                );
                prog.push(Op {
                    mnem: op,
                    reg: reg,
                    offset: offset,
                });
            }
            _ => bail!("Invalid opcode: {}", op),
        }
    }

    Ok(prog)
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let program = input_to_program(input)?;
    let mut state = State { ip: 0, a: 0, b: 0 };
    while state.ip < program.len() {
        exec(program[state.ip], &mut state);
    }
    Ok(state.b as i32)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let program = input_to_program(input)?;
    let mut state = State { ip: 0, a: 1, b: 0 };
    while state.ip < program.len() {
        exec(program[state.ip], &mut state);
    }
    Ok(state.b as i32)
}
