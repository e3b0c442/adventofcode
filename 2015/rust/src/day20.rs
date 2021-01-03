use simple_error::bail;
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day20(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 20: Infinite Elves and Infinite Houses");

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
    let target = input.parse::<usize>()?;
    let mut houses = vec![0; target / 10 + 1];
    for i in 1..=(target / 10) {
        for j in (i..=(target / 10)).step_by(i) {
            houses[j] += i * 10;
        }
    }
    for (i, house) in houses.into_iter().enumerate() {
        if house > target {
            return Ok(i as i32);
        }
    }
    bail!("Solution not found");
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let target = input.parse::<usize>()?;
    let mut houses = vec![0; target / 11 + 1];
    for i in 1..=(target / 11) {
        for j in (i..=min(i * 50, target / 11)).step_by(i) {
            houses[j] += i * 11;
        }
    }
    for (i, house) in houses.into_iter().enumerate() {
        if house > target {
            return Ok(i as i32);
        }
    }
    bail!("Solution not found");
}
