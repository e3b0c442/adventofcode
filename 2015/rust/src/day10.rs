use simple_error::require_with;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day10(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 10: Elves Look, Elves Say");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let start = Instant::now();
    let (sol1, cont) = part1(&input)?;
    println!("\tPart 1: {} ({:?})", sol1, start.elapsed());
    let second = Instant::now();
    println!("\tPart 2: {} ({:?})", part2(&cont)?, second.elapsed());
    println!("\t\t Completed in {:?}", start.elapsed());
    Ok(())
}

fn part1(input: &str) -> Result<(i32, String), Box<dyn Error>> {
    let mut input = input.to_owned();
    for _ in 0..40 {
        let mut output = String::new();
        let mut iter = input.chars();
        let mut last = require_with!(iter.next(), &format!("Invalid input: {}", input));
        let mut count = 1;
        while let Some(num) = iter.next() {
            if num == last {
                count += 1;
            } else {
                output.push_str(&format!("{}{}", count, last));
                count = 1;
                last = num;
            }
        }
        output.push_str(&format!("{}{}", count, last));
        input = output;
    }
    return Ok((input.len() as i32, input));
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut input = input.to_owned();
    for _ in 40..50 {
        let mut output = String::new();
        let mut iter = input.chars();
        let mut last = require_with!(iter.next(), &format!("Invalid input: {}", input));
        let mut count = 1;
        while let Some(num) = iter.next() {
            if num == last {
                count += 1;
            } else {
                output.push_str(&format!("{}{}", count, last));
                count = 1;
                last = num;
            }
        }
        output.push_str(&format!("{}{}", count, last));
        input = output;
    }
    return Ok(input.len() as i32);
}
