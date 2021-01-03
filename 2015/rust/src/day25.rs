use lazy_static::lazy_static;
use regex::Regex;
use simple_error::require_with;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day25(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 25: Let It Snow");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let start = Instant::now();
    println!("\tPart 1: {} ({:?})", part1(&input)?, start.elapsed());
    println!("\t\t Completed in {:?}", start.elapsed());
    Ok(())
}

lazy_static! {
    static ref COORDS_RE: Regex = Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).").unwrap();
}

fn input_to_coordinates(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let caps = require_with!(COORDS_RE.captures(input), "Invalid input");
    Ok((
        require_with!(caps.get(1), "Invalid input")
            .as_str()
            .parse::<usize>()?,
        require_with!(caps.get(2), "Invalid input")
            .as_str()
            .parse::<usize>()?,
    ))
}

fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let (row, col) = input_to_coordinates(input)?;
    let mut val = 20151125;
    let mut diag = 2;
    loop {
        let (mut x, mut y) = (1, diag);
        while y > 0 {
            val = (val * 252533) % 33554393;
            if x == col && y == row {
                return Ok(val);
            }
            x += 1;
            y -= 1;
        }
        diag += 1;
    }
}
