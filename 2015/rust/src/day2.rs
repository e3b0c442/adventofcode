use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day2(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 2: I Was Told There Would Be No Math");

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

fn input_to_dims(input: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut dims = input
        .lines()
        .map(|line| {
            line.split("x")
                .map(|d| d.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()?;
    dims.sort();
    Ok(dims)
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input_to_dims(input)?.iter().fold(0, |paper, dims| {
        paper
            + 2 * dims[0] * dims[1]
            + 2 * dims[1] * dims[2]
            + 2 * dims[2] * dims[0]
            + dims[0] * dims[1]
    }))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input_to_dims(input)?.iter().fold(0, |ribbon, dims| {
        ribbon + 2 * dims[0] + 2 * dims[1] + dims[0] * dims[1] * dims[2]
    }))
}
