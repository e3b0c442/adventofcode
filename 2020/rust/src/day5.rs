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
        let rowvec = (0..128).collect::<Vec<i32>>();
        let mut rows: &[i32] = rowvec.as_slice();
        for c in line[0..7].chars() {
            let split = rows.len() / 2;
            match c {
                'F' => rows = &rows[0..split],
                'B' => rows = &rows[split..],
                _ => return Err(Box::new(SimpleError::new("Invalid input"))),
            }
        }
        let row = rows[0];

        let colvec = (0..8).collect::<Vec<i32>>();
        let mut cols: &[i32] = colvec.as_slice();
        for c in line[7..].chars() {
            let split = cols.len() / 2;
            match c {
                'L' => cols = &cols[0..split],
                'R' => cols = &cols[split..],
                _ => return Err(Box::new(SimpleError::new("Invalid input"))),
            }
        }
        let col = cols[0];
        ids.push(row * 8 + col);
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
