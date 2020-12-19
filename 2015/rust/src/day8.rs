use simple_error::{bail, require_with};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day8(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 8: Matchsticks");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut code = 0;
    let mut string = 0;
    for line in input.lines() {
        let mut in_str = false;
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    code += 1;
                    in_str = !in_str;
                }
                '\\' => {
                    code += 1;
                    if in_str {
                        let c = require_with!(chars.next(), "bad");
                        match c {
                            '\\' | '"' => {
                                code += 1;
                                string += 1;
                            }
                            'x' => {
                                code += 3;
                                string += 1;
                                chars.next();
                                chars.next();
                            }
                            _ => {
                                bail!(format!("Invalid input: {}", line));
                            }
                        }
                    }
                }
                _ => {
                    code += 1;
                    if in_str {
                        string += 1;
                    }
                }
            }
        }
    }
    Ok(code - string)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut encoded = 0;
    let mut string = 0;
    for line in input.lines() {
        encoded += 2;
        for c in line.chars() {
            string += 1;
            match c {
                '\\' | '"' => encoded += 2,
                _ => encoded += 1,
            }
        }
    }
    Ok(encoded - string)
}
