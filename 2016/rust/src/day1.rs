use simple_error::{bail, SimpleError};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day1(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 1: No Time for a Taxicab");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let parts = input.split(", ").collect::<Vec<&str>>();

    let mut heading = 0;
    let (mut x, mut y) = (0, 0);

    for part in parts {
        let (turn, dist) = (&part[0..1], (&part[1..]).parse::<i32>()?);
        match turn {
            "L" => heading -= 90,
            "R" => heading += 90,
            _ => bail!("Invalid input: {}", part),
        }
        if heading < 0 {
            heading += 360;
        } else if heading >= 360 {
            heading -= 360;
        }
        match heading {
            0 => y += dist,
            90 => x += dist,
            180 => y -= dist,
            270 => x -= dist,
            _ => bail!("Invalid input: {}", part),
        }
    }
    Ok(x.abs() + y.abs())
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let parts = input.split(", ").collect::<Vec<&str>>();

    let mut heading = 0;
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut visited = HashSet::new();

    visited.insert((x, y));
    for part in parts {
        let (turn, dist) = (&part[0..1], (&part[1..]).parse::<i32>()?);
        match turn {
            "L" => heading -= 90,
            "R" => heading += 90,
            _ => bail!("Invalid input: {}", part),
        }
        if heading < 0 {
            heading += 360;
        } else if heading >= 360 {
            heading -= 360;
        }
        if let Err(_) = match heading {
            0 => (0..dist).try_for_each(|_| {
                y += 1;
                if !visited.insert((x, y)) {
                    return Err(SimpleError::new(""));
                }
                Ok(())
            }),
            90 => (0..dist).try_for_each(|_| {
                x += 1;
                if !visited.insert((x, y)) {
                    return Err(SimpleError::new(""));
                }
                Ok(())
            }),
            180 => (0..dist).try_for_each(|_| {
                y -= 1;
                if !visited.insert((x, y)) {
                    return Err(SimpleError::new(""));
                }
                Ok(())
            }),
            270 => (0..dist).try_for_each(|_| {
                x -= 1;
                if !visited.insert((x, y)) {
                    return Err(SimpleError::new(""));
                }
                Ok(())
            }),
            _ => bail!("Invalid input: {}", part),
        } {
            return Ok(x.abs() + y.abs());
        }
    }
    bail!("Solution not found");
}
