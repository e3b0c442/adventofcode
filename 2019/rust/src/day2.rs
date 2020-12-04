use simple_error::SimpleError;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day2(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 2: 1202 Program Alarm");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn intcode_from_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn run_intcode(mut intcode: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    let mut cursor: usize = 0;
    loop {
        let (l, r, t) = (
            intcode[cursor + 1] as usize,
            intcode[cursor + 2] as usize,
            intcode[cursor + 3] as usize,
        );
        match intcode[cursor] {
            1 => intcode[t] = intcode[l] + intcode[r],
            2 => intcode[t] = intcode[l] * intcode[r],
            99 => break,
            _ => {
                return Err(Box::new(SimpleError::new(format!(
                    "Invalid opcode: {}",
                    intcode[cursor]
                ))))
            }
        }
        cursor += 4;
    }
    Ok(intcode[0])
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut intcode = intcode_from_input(input).clone();
    intcode[1] = 12;
    intcode[2] = 2;
    run_intcode(intcode)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut intcode = intcode_from_input(input);
    for i in 0..100 {
        for j in 0..100 {
            intcode[1] = i;
            intcode[2] = j;
            if run_intcode(intcode.clone())? == 19690720 {
                return Ok(100 * i + j);
            }
        }
    }
    Err(Box::new(SimpleError::new("Solution not found")))
}
