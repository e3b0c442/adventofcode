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
    let groups = input.split("\n\n");
    let mut sum = 0;
    for group in groups {
        let mut set: HashSet<char> = HashSet::new();
        for line in group.lines() {
            for c in line.chars() {
                set.insert(c);
            }
        }
        sum += set.len()
    }
    Ok(sum as i32)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let groups = input.split("\n\n");
    let mut sum = 0;
    for group in groups {
        let mut map: HashMap<char, usize> = HashMap::new();
        for line in group.lines() {
            for c in line.chars() {
                let counter = map.entry(c).or_insert(0);
                *counter += 1;
            }
        }
        let group_size = group.lines().count();
        sum += map.iter().filter(|&(_, v)| *v == group_size).count();
    }
    Ok(sum as i32)
}
