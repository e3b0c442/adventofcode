use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

lazy_static! {
    static ref PASS_RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*)").unwrap();
}

pub fn day2(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 2: Password Philosophy");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut valid = 0;
    for line in input.lines() {
        let caps = PASS_RE.captures(line).unwrap();
        let (min, max, ch, pw) = (
            caps.get(1).unwrap().as_str().parse::<i32>()?,
            caps.get(2).unwrap().as_str().parse::<i32>()?,
            caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            caps.get(4).unwrap().as_str(),
        );

        let mut count = 0;
        for c in pw.chars() {
            if c == ch {
                count += 1;
            }
        }
        if min <= count && count <= max {
            valid += 1;
        }
    }
    Ok(valid)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut valid = 0;
    for line in input.lines() {
        let caps = PASS_RE.captures(line).unwrap();
        let (l, r, c, pw) = (
            caps.get(1).unwrap().as_str().parse::<usize>()? - 1,
            caps.get(2).unwrap().as_str().parse::<usize>()? - 1,
            caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            caps.get(4).unwrap().as_str(),
        );
        if (pw.chars().nth(l) == Some(c) || pw.chars().nth(r) == Some(c))
            && pw.chars().nth(l) != pw.chars().nth(r)
        {
            valid += 1;
        }
    }
    Ok(valid)
}
