use lazy_static::lazy_static;
use regex::Regex;
use simple_error::SimpleError;
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
    input.lines().try_fold(0, |valid, line| {
        if let Some(caps) = PASS_RE.captures(line) {
            let min: i32;
            if let Some(min_c) = caps.get(1) {
                min = min_c.as_str().parse::<i32>()?;
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }
            let max: i32;
            if let Some(max_c) = caps.get(2) {
                max = max_c.as_str().parse::<i32>()?;
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }
            let ch: char;
            if let Some(ch_c) = caps.get(3) {
                if let Some(ch_o) = ch_c.as_str().chars().nth(0) {
                    ch = ch_o;
                } else {
                    return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
                }
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }
            let pw: &str;
            if let Some(pw_c) = caps.get(4) {
                pw = pw_c.as_str();
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }

            let count = pw
                .chars()
                .fold(0, |sum, c| if c == ch { sum + 1 } else { sum });
            if min <= count && count <= max {
                Ok(valid + 1)
            } else {
                Ok(valid)
            }
        } else {
            Err(SimpleError::new(format!("Solution not found: {}", line)).into())
        }
    })
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    input.lines().try_fold(0, |valid, line| {
        if let Some(caps) = PASS_RE.captures(line) {
            let l: usize;
            if let Some(l_c) = caps.get(1) {
                l = l_c.as_str().parse::<usize>()? - 1;
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }
            let r: usize;
            if let Some(r_c) = caps.get(2) {
                r = r_c.as_str().parse::<usize>()? - 1;
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }
            let c: char;
            if let Some(c_c) = caps.get(3) {
                if let Some(c_o) = c_c.as_str().chars().nth(0) {
                    c = c_o;
                } else {
                    return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
                }
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }
            let pw: Vec<char>;
            if let Some(pw_c) = caps.get(4) {
                pw = pw_c.as_str().chars().collect();
            } else {
                return Err(SimpleError::new(format!("Invalid input: {}", line)).into());
            }

            if (pw[l] == c || pw[r] == c) && pw[l] != pw[r] {
                Ok(valid + 1)
            } else {
                Ok(valid)
            }
        } else {
            Err(SimpleError::new(format!("Solution not found: {}", line)).into())
        }
    })
}
