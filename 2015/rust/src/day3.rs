use simple_error::SimpleError;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day3(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 3: Perfectly Spherical Houses in a Vacuum");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    for c in input.chars() {
        match c {
            '>' => {
                x += 1;
                if x > max_x {
                    max_x = x;
                }
            }
            'v' => {
                y -= 1;
                if y < min_y {
                    min_y = y;
                }
            }
            '<' => {
                x -= 1;
                if x < min_x {
                    min_x = x;
                }
            }
            '^' => {
                y += 1;
                if y > max_y {
                    max_y = y;
                }
            }
            _ => return Err(SimpleError::new(format!("Invalid input: {}", c)).into()),
        };
    }

    let (w, h) = ((max_x + 1 - min_x) as usize, (max_y + 1 - min_y) as usize);
    let mut grid = vec![vec![false; h]; w];
    let mut x = -min_x as usize;
    let mut y = -min_y as usize;
    grid[x][y] = true;
    for c in input.chars() {
        match c {
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '^' => y += 1,
            _ => return Err(SimpleError::new(format!("Invalid input: {}", c)).into()),
        };
        grid[x][y] = true;
    }

    Ok(grid.iter().fold(0, |sum, row| {
        sum + row.iter().fold(0, |s, c| if *c { s + 1 } else { s })
    }))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut x: [i32; 2] = [0, 0];
    let mut y: [i32; 2] = [0, 0];
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut s: usize = 0;
    for c in input.chars() {
        match c {
            '>' => {
                x[s] += 1;
                if x[s] > max_x {
                    max_x = x[s];
                }
            }
            'v' => {
                y[s] -= 1;
                if y[s] < min_y {
                    min_y = y[s];
                }
            }
            '<' => {
                x[s] -= 1;
                if x[s] < min_x {
                    min_x = x[s];
                }
            }
            '^' => {
                y[s] += 1;
                if y[s] > max_y {
                    max_y = y[s];
                }
            }
            _ => return Err(SimpleError::new(format!("Invalid input: {}", c)).into()),
        };
        s ^= 1;
    }

    let (w, h) = ((max_x + 1 - min_x) as usize, (max_y + 1 - min_y) as usize);
    let mut grid = vec![vec![false; h]; w];
    let mut x: [usize; 2] = [-min_x as usize, -min_x as usize];
    let mut y: [usize; 2] = [-min_y as usize, -min_y as usize];
    s = 0;
    grid[x[s]][y[s]] = true;
    for c in input.chars() {
        match c {
            '>' => x[s] += 1,
            'v' => y[s] -= 1,
            '<' => x[s] -= 1,
            '^' => y[s] += 1,
            _ => return Err(SimpleError::new(format!("Invalid input: {}", c)).into()),
        };
        grid[x[s]][y[s]] = true;
        s ^= 1;
    }

    Ok(grid.iter().fold(0, |sum, row| {
        sum + row.iter().fold(0, |s, c| if *c { s + 1 } else { s })
    }))
}
