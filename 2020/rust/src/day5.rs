use simple_error::SimpleError;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day5(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 5: Binary Boarding");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let ids = part1(&input)?;
    println!("\tPart 1: {}", ids.last().unwrap());
    println!("\tPart 2: {}", part2(ids)?);
    Ok(())
}

fn part1(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut ids: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut row = (0, 127);
        for c in line[0..7].chars() {
            let split = (row.1 + 1 - row.0) / 2;
            match c {
                'F' => row.1 -= split,
                'B' => row.0 += split,
                _ => return Err(Box::new(SimpleError::new("Invalid input"))),
            }
        }

        let mut col = (0, 7);
        for c in line[7..].chars() {
            let split = (col.1 + 1 - col.0) / 2;
            match c {
                'L' => col.1 -= split,
                'R' => col.0 += split,
                _ => return Err(Box::new(SimpleError::new("Invalid input"))),
            }
        }
        ids.push(row.0 * 8 + col.0);
    }
    ids.sort();
    Ok(ids)
}

fn part2(ids: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    for i in 0..ids.len() - 2 {
        if ids[i] + 1 != ids[i + 1] {
            return Ok(ids[i] + 1);
        }
    }
    Err(Box::new(SimpleError::new("Solution not found")))
}
