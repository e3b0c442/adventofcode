use simple_error::bail;
use std::cmp::{max, min};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day3(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 3: Perfectly Spherical Houses in a Vacuum");

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

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let (mut x, mut y) = (0, 0);
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
    for c in input.chars() {
        match c {
            '>' => {
                x += 1;
                max_x = max(x, max_x);
            }
            'v' => {
                y -= 1;
                min_y = min(y, min_y);
            }
            '<' => {
                x -= 1;
                min_x = min(x, min_x);
            }
            '^' => {
                y += 1;
                max_y = max(y, max_y);
            }
            _ => bail!("Invalid input: {}", c),
        };
    }

    let (w, h) = ((max_x + 1 - min_x) as usize, (max_y + 1 - min_y) as usize);
    let mut grid = vec![vec![false; h]; w];
    let (mut x, mut y) = (-min_x as usize, -min_y as usize);
    grid[x][y] = true;

    for c in input.chars() {
        match c {
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '^' => y += 1,
            _ => bail!("Invalid input: {}", c),
        };
        grid[x][y] = true;
    }

    Ok(grid.iter().fold(0, |sum, row| {
        sum + row.iter().fold(0, |s, c| if *c { s + 1 } else { s })
    }))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let (mut x, mut y) = ([0, 0], [0, 0]);
    let (mut min_x, mut max_x, mut min_y, mut max_y, mut s) = (0, 0, 0, 0, 0);
    for c in input.chars() {
        match c {
            '>' => {
                x[s] += 1;
                max_x = max(x[s], max_x);
            }
            'v' => {
                y[s] -= 1;
                min_y = min(y[s], min_y);
            }
            '<' => {
                x[s] -= 1;
                min_x = min(x[s], min_x);
            }
            '^' => {
                y[s] += 1;
                max_y = max(y[s], max_y);
            }
            _ => bail!(format!("Invalid input: {}", c)),
        };
        s ^= 1;
    }

    let (w, h) = ((max_x + 1 - min_x) as usize, (max_y + 1 - min_y) as usize);
    let mut grid = vec![vec![false; h]; w];
    let (mut x, mut y) = (
        [-min_x as usize, -min_x as usize],
        [-min_y as usize, -min_y as usize],
    );
    s = 0;
    grid[x[s]][y[s]] = true;
    for c in input.chars() {
        match c {
            '>' => x[s] += 1,
            'v' => y[s] -= 1,
            '<' => x[s] -= 1,
            '^' => y[s] += 1,
            _ => bail!("Invalid input: {}", c),
        };
        grid[x[s]][y[s]] = true;
        s ^= 1;
    }

    Ok(grid.iter().fold(0, |sum, row| {
        sum + row.iter().fold(0, |s, c| if *c { s + 1 } else { s })
    }))
}
