use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{bail, require_with};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day6(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 6: Probably a Fire Hazard");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

lazy_static! {
    static ref INSTR_RE: Regex =
        Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut grid = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let caps = require_with!(INSTR_RE.captures(line), &format!("Invalid input: {}", line));
        let (instr, min_x, max_x, min_y, max_y) = (
            require_with!(caps.get(1), &format!("Invalid input: {}", line)).as_str(),
            require_with!(caps.get(2), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
            require_with!(caps.get(3), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
            require_with!(caps.get(4), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
            require_with!(caps.get(5), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
        );
        match instr {
            "turn on" => {
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        grid[x][y] = 1;
                    }
                }
            }
            "turn off" => {
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        grid[x][y] = 0;
                    }
                }
            }
            "toggle" => {
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        grid[x][y] ^= 1;
                    }
                }
            }
            _ => bail!(format!("Invalid input: {}", line)),
        }
    }
    Ok(grid
        .iter()
        .fold(0, |sum, row| sum + row.iter().fold(0, |s, c| s + c)))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut grid = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let caps = require_with!(INSTR_RE.captures(line), &format!("Invalid input: {}", line));
        let (instr, min_x, max_x, min_y, max_y) = (
            require_with!(caps.get(1), &format!("Invalid input: {}", line)).as_str(),
            require_with!(caps.get(2), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
            require_with!(caps.get(3), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
            require_with!(caps.get(4), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
            require_with!(caps.get(5), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<usize>()?,
        );
        match instr {
            "turn on" => {
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        grid[x][y] += 1;
                    }
                }
            }
            "turn off" => {
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        if grid[x][y] > 0 {
                            grid[x][y] -= 1;
                        }
                    }
                }
            }
            "toggle" => {
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        grid[x][y] += 2;
                    }
                }
            }
            _ => bail!(format!("Invalid input: {}", line)),
        }
    }
    Ok(grid
        .iter()
        .fold(0, |sum, row| sum + row.iter().fold(0, |s, c| s + c)))
}
