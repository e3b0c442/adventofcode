use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day17(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 17: No Such Thing as Too Much");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let start = Instant::now();
    let sol1 = part1(&input)?;
    println!("\tPart 1: {} ({:?})", sol1.0, start.elapsed());
    let second = Instant::now();
    println!(
        "\tPart 2: {} ({:?})",
        part2(&input, sol1.1)?,
        second.elapsed()
    );
    println!("\t\t Completed in {:?}", start.elapsed());
    Ok(())
}

fn find_container_combos(
    containers: Vec<i32>,
    target: i32,
    depth: i32,
    target_depth: i32,
    mut smallest: i32,
) -> (i32, i32) {
    if containers.len() == 0 || (target_depth > 0 && depth > target_depth) {
        return (0, i32::MAX);
    }

    let mut found = 0;
    for (i, c) in containers.iter().enumerate() {
        if *c > target {
            continue;
        }
        if *c == target && (target_depth <= 0 || depth + 1 == target_depth) {
            found += 1;
            smallest = depth + 1;
            continue;
        }
        let mut containers = containers.clone();
        containers.remove(i);
        let containers = (&containers[i..]).iter().map(|x| *x).collect::<Vec<i32>>();
        let (leg_found, leg_min) =
            find_container_combos(containers, target - *c, depth + 1, target_depth, smallest);
        found += leg_found;
        smallest = min(smallest, leg_min);
    }

    (found, smallest)
}

fn part1(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let mut sizes = input
        .lines()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    sizes.sort();
    sizes = sizes.into_iter().rev().collect();

    Ok(find_container_combos(sizes, 150, 0, -1, i32::MAX))
}

fn part2(input: &str, size: i32) -> Result<i32, Box<dyn Error>> {
    let mut sizes = input
        .lines()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    sizes.sort();
    sizes = sizes.into_iter().rev().collect();

    Ok(find_container_combos(sizes, 150, 0, size, i32::MAX).0)
}
