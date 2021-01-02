use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day3(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 3: Squares With Three Sides");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()?
        .iter()
        .fold(0, |mut sum, dims| {
            if dims[0] + dims[1] > dims[2]
                && dims[1] + dims[2] > dims[0]
                && dims[2] + dims[0] > dims[1]
            {
                sum += 1
            }
            sum
        }))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut rows = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()?;
    for i in (0..rows.len()).step_by(3) {
        for pair in [
            ((i + 1, 0), (i + 0, 1)),
            ((i + 2, 0), (i + 0, 2)),
            ((i + 2, 1), (i + 1, 2)),
        ]
        .iter()
        {
            let (l, r) = pair;
            let t = rows[l.0][l.1];
            rows[l.0][l.1] = rows[r.0][r.1];
            rows[r.0][r.1] = t;
        }
    }
    Ok(rows.iter().fold(0, |mut sum, dims| {
        if dims[0] + dims[1] > dims[2] && dims[1] + dims[2] > dims[0] && dims[2] + dims[0] > dims[1]
        {
            sum += 1
        }
        sum
    }))
}
