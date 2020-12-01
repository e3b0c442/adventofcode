use simple_error::SimpleError;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day1(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 1: Report Repair");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut entries = Vec::new();
    for line in input.lines() {
        entries.push(line.parse::<i32>()?);
    }
    for i in 0..entries.len() {
        for j in i + 1..entries.len() {
            if entries[i] + entries[j] == 2020 {
                return Ok(entries[i] * entries[j]);
            }
        }
    }

    Err(Box::new(SimpleError::new("Solution not found")))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut entries = Vec::new();
    for line in input.lines() {
        entries.push(line.parse::<i32>()?);
    }
    for i in 0..entries.len() {
        for j in i + 1..entries.len() {
            for k in j + 1..entries.len() {
                if entries[i] + entries[j] + entries[k] == 2020 {
                    return Ok(entries[i] * entries[j] * entries[k]);
                }
            }
        }
    }

    Err(Box::new(SimpleError::new("Solution not found")))
}
