use simple_error::require_with;
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day24(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 24: It Hangs in the Balance");

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

fn make_smallest_groups(
    items: Vec<i32>,
    pool: Vec<i32>,
    target: i32,
    mut max_len: usize,
) -> Vec<Vec<i32>> {
    let len = pool.len();
    if len == 0 || items.len() >= max_len {
        return Vec::new();
    }
    let sum: i32 = items.iter().sum();

    let mut groups = Vec::new();
    for i in 0..len {
        if pool[i] > target - sum {
            continue;
        }
        let mut v = items.clone();
        v.push(pool[i]);

        if v.iter().sum::<i32>() == target {
            max_len = min(max_len, v.len());
            groups.push(v);
            continue;
        }

        groups.extend(make_smallest_groups(
            v,
            (&pool[i + 1..]).iter().map(|x| *x).collect::<Vec<i32>>(),
            target,
            max_len,
        ));

        groups.sort_by(|a, b| a.len().cmp(&b.len()));
        match groups.get(0) {
            Some(g) => max_len = min(max_len, g.len()),
            None => (),
        }
        groups = groups.into_iter().filter(|x| x.len() == max_len).collect();
    }

    groups
}

fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut weights = input
        .lines()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()?;

    weights.sort();
    let weights = weights.into_iter().rev().collect::<Vec<i32>>();

    let total_weight: i32 = weights.iter().sum();
    let compartment_weight = total_weight / 3;

    let possible_groups = make_smallest_groups(Vec::new(), weights, compartment_weight, usize::MAX);
    Ok(require_with!(
        possible_groups
            .into_iter()
            .map(|x| x.iter().map(|x| *x as i64).product::<i64>())
            .min(),
        "Solution not found"
    ))
}

fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut weights = input
        .lines()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()?;

    weights.sort();
    let weights = weights.into_iter().rev().collect::<Vec<i32>>();

    let total_weight: i32 = weights.iter().sum();
    let compartment_weight = total_weight / 4;

    let possible_groups = make_smallest_groups(Vec::new(), weights, compartment_weight, usize::MAX);
    Ok(require_with!(
        possible_groups
            .into_iter()
            .map(|x| x.iter().map(|x| *x as i64).product::<i64>())
            .min(),
        "Solution not found"
    ))
}
