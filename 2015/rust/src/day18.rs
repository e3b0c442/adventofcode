use simple_error::bail;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day18(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 18: Like a GIF For Your Yard");

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

fn input_to_grid(input: &str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    '#' => Ok(true),
                    '.' => Ok(false),
                    _ => bail!("Invalid input: {}", x),
                })
                .collect()
        })
        .collect()
}

fn count_neighbors(grid: &Vec<Vec<bool>>, dim: usize, x: usize, y: usize) -> i32 {
    let mut count = 0;
    if x > 0 {
        if grid[x - 1][y] {
            count += 1;
        }
    }
    if x > 0 && y > 0 {
        if grid[x - 1][y - 1] {
            count += 1;
        }
    }
    if y > 0 {
        if grid[x][y - 1] {
            count += 1;
        }
    }
    if y > 0 && x < dim - 1 {
        if grid[x + 1][y - 1] {
            count += 1
        }
    }
    if x < dim - 1 {
        if grid[x + 1][y] {
            count += 1;
        }
    }
    if x < dim - 1 && y < dim - 1 {
        if grid[x + 1][y + 1] {
            count += 1;
        }
    }
    if y < dim - 1 {
        if grid[x][y + 1] {
            count += 1;
        }
    }
    if y < dim - 1 && x > 0 {
        if grid[x - 1][y + 1] {
            count += 1;
        }
    }

    count
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut grid = input_to_grid(input)?;

    for _ in 0..100 {
        let mut new_grid = grid.clone();
        for x in 0..100 {
            for y in 0..100 {
                let n = count_neighbors(&grid, 100, x, y);
                if grid[x][y] && n != 2 && n != 3 {
                    new_grid[x][y] = false;
                } else if !grid[x][y] && n == 3 {
                    new_grid[x][y] = true;
                }
            }
        }
        grid = new_grid;
    }

    Ok(grid.into_iter().fold(0, |sum, x| {
        sum + x.into_iter().fold(0, |s, y| if y { s + 1 } else { s })
    }))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut grid = input_to_grid(input)?;

    for _ in 0..100 {
        let mut new_grid = grid.clone();
        for x in 0..100 {
            for y in 0..100 {
                let n = count_neighbors(&grid, 100, x, y);
                if grid[x][y] && n != 2 && n != 3 {
                    new_grid[x][y] = false;
                } else if !grid[x][y] && n == 3 {
                    new_grid[x][y] = true;
                }
            }
        }
        new_grid[0][0] = true;
        new_grid[0][99] = true;
        new_grid[99][99] = true;
        new_grid[99][0] = true;
        grid = new_grid;
    }

    Ok(grid.into_iter().fold(0, |sum, x| {
        sum + x.into_iter().fold(0, |s, y| if y { s + 1 } else { s })
    }))
}
