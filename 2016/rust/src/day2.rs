use simple_error::{bail, require_with};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day2(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 2: Bathroom Security");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn find_code(input: &str, keymap: &HashMap<(i32, i32), &str>) -> Result<String, Box<dyn Error>> {
    let (mut x, mut y) = (0, 0);
    let mut code = String::new();
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => {
                    if keymap.contains_key(&(x, y + 1)) {
                        y += 1
                    }
                }
                'R' => {
                    if keymap.contains_key(&(x + 1, y)) {
                        x += 1
                    }
                }
                'D' => {
                    if keymap.contains_key(&(x, y - 1)) {
                        y -= 1
                    }
                }
                'L' => {
                    if keymap.contains_key(&(x - 1, y)) {
                        x -= 1
                    }
                }
                _ => bail!("Invalid input: {}", line),
            };
        }
        code += require_with!(keymap.get(&(x, y)), &format!("Bad coordinate {:?}", (x, y)));
    }
    Ok(code)
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let keymap: HashMap<(i32, i32), &str> = [
        ((-1, 1), "1"),
        ((0, 1), "2"),
        ((1, 1), "3"),
        ((-1, 0), "4"),
        ((0, 0), "5"),
        ((1, 0), "6"),
        ((-1, -1), "7"),
        ((0, -1), "8"),
        ((1, -1), "9"),
    ]
    .iter()
    .cloned()
    .collect();

    Ok(find_code(input, &keymap)?)
}

fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let keymap: HashMap<(i32, i32), &str> = [
        ((0, 2), "1"),
        ((-1, 1), "2"),
        ((0, 1), "3"),
        ((1, 1), "4"),
        ((-2, 0), "5"),
        ((-1, 0), "6"),
        ((0, 0), "7"),
        ((1, 0), "8"),
        ((2, 0), "9"),
        ((-1, -1), "A"),
        ((0, -1), "B"),
        ((1, -1), "C"),
        ((0, -2), "D"),
    ]
    .iter()
    .cloned()
    .collect();

    Ok(find_code(input, &keymap)?)
}
