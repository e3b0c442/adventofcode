use simple_error::SimpleError;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day8(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 8: Handheld Halting");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let instrs: Vec<(&str, i32)> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (parts[0], parts[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut acc = 0;
    let mut ip = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    loop {
        if visited.contains(&ip) {
            return Ok(acc);
        }
        visited.insert(ip);
        let op = instrs[ip as usize];
        match op.0 {
            "acc" => {
                acc += op.1;
                ip += 1;
            }
            "jmp" => {
                ip += op.1;
            }
            "nop" => {
                ip += 1;
            }
            _ => return Err(Box::new(SimpleError::new("Invalid input"))),
        }
    }
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let unmodified: Vec<(&str, i32)> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (parts[0], parts[1].parse::<i32>().unwrap())
        })
        .collect();

    'outer: for i in 0..unmodified.len() {
        let mut instrs = unmodified.clone();
        match instrs[i].0 {
            "acc" => continue,
            "jmp" => instrs[i] = ("nop", instrs[i].1),
            "nop" => instrs[i] = ("jmp", instrs[i].1),
            _ => return Err(Box::new(SimpleError::new("Invalid input"))),
        };

        let mut acc = 0;
        let mut ip = 0;
        let mut visited: HashSet<i32> = HashSet::new();
        let instrslen = instrs.len() as i32;
        loop {
            if ip == instrslen {
                return Ok(acc);
            }
            if visited.contains(&ip) {
                continue 'outer;
            }
            visited.insert(ip);
            let op = instrs[ip as usize];
            match op.0 {
                "acc" => {
                    acc += op.1;
                    ip += 1;
                }
                "jmp" => {
                    ip += op.1;
                }
                "nop" => {
                    ip += 1;
                }
                _ => return Err(Box::new(SimpleError::new("Invalid input"))),
            }
        }
    }
    Ok(0)
}
