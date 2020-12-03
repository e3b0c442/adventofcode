use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day1(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 1: The Tyranny of the Rocket Equation");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn recurse_fuel(val: i32) -> i32 {
    if val / 3 - 2 <= 0 {
        0
    } else {
        val / 3 - 2 + recurse_fuel(val / 3 - 2)
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .fold(0, |acc, cur| acc + cur / 3 - 2))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .fold(0, |acc, cur| acc + recurse_fuel(cur)))
}
