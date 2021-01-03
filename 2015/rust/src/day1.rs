use simple_error::{bail, SimpleError};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day1(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 1: Not Quite Lisp");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let start = Instant::now();
    println!("\tPart 1: {} ({:?})", part1(&input)?, start.elapsed());
    let second = Instant::now();
    println!("\tPart 2: {} ({:?})", part2(&input)?, second.elapsed());
    println!("\t\t Completed in {:?}", start.elapsed());
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    input.chars().try_fold(0, |floor, c| match c {
        '(' => Ok(floor + 1),
        ')' => Ok(floor - 1),
        _ => bail!("Invalid input: {}", c),
    })
}

#[derive(Debug)]
enum Basement {
    Floor(i32),
    Err(Box<dyn Error>),
}
impl std::error::Error for Basement {}
impl std::fmt::Display for Basement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    match input
        .chars()
        .enumerate()
        .try_fold(0, |mut floor, (step, c)| match c {
            '(' => Ok(floor + 1),
            ')' => {
                floor -= 1;
                if floor < 0 {
                    Err(Basement::Floor(step as i32))
                } else {
                    Ok(floor)
                }
            }
            _ => Err(Basement::Err(SimpleError::new("Invalid input").into())),
        }) {
        Err(Basement::Floor(step)) => Ok(step),
        _ => bail!("Solution not found"),
    }
}
