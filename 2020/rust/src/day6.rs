use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day6(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 6: Custom Customs");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input.split("\n\n").fold(0, |sum, group| {
        sum + group
            .lines()
            .fold(HashSet::new(), |set, line| {
                line.chars().fold(set, |mut qs, c| {
                    qs.insert(c);
                    qs
                })
            })
            .len() as i32
    }))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input.split("\n\n").fold(0, |sum, group| {
        let group_size = group.lines().count();
        sum + group
            .lines()
            .fold(HashMap::new(), |map, line| {
                line.chars().fold(map, |mut qs, c| {
                    let counter = qs.entry(c).or_insert(0);
                    *counter += 1;
                    qs
                })
            })
            .iter()
            .filter(|&(_, v)| *v == group_size)
            .count() as i32
    }))
}
