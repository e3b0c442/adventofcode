use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day3(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 3: Toboggan Trajectory");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn slope_trees(rows: Vec<Vec<char>>, slope: (usize, usize)) -> i32 {
    let (r, d) = slope;
    let (mut trees, mut x) = (0, 0);
    let mut rows = rows.into_iter().step_by(d);
    while let Some(row) = rows.next() {
        if x >= row.len() {
            x = x - row.len()
        }
        if row[x] == '#' {
            trees += 1;
        }
        x += r
    }
    trees
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(slope_trees(
        input
            .lines()
            .map(|x| x.chars().collect())
            .collect::<Vec<Vec<char>>>(),
        (3, 1),
    ))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    Ok(slopes.iter().fold(1, |acc, cur| {
        acc * slope_trees(
            input
                .lines()
                .map(|x| x.chars().collect())
                .collect::<Vec<Vec<char>>>(),
            *cur,
        )
    }))
}
