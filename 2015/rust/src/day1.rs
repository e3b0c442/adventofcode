use simple_error::bail;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day1(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 1: Not Quite Lisp");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    input.chars().try_fold(0, |floor, c| match c {
        '(' => Ok(floor + 1),
        ')' => Ok(floor - 1),
        _ => bail!(format!("Invalid input: {}", c)),
    })
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => bail!(format!("Invalid input: {}", c)),
        };
        if floor == -1 {
            return Ok((i + 1) as i32);
        }
    }
    bail!("Solution not found")
}
