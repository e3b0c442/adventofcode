use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day6(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 6: Signals and Noise");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let width = input.lines().nth(0).unwrap().len();
    let mut occurrences = (0..width)
        .map(|_| HashMap::new())
        .collect::<Vec<HashMap<char, i32>>>();
    input.lines().for_each(|x| {
        x.chars().enumerate().for_each(|(i, c)| {
            let e = occurrences[i].entry(c).or_insert(0);
            *e += 1;
        })
    });
    Ok(occurrences
        .iter()
        .map(|x| *(x.iter().max_by_key(|(_, count)| *count).unwrap().0))
        .collect::<String>())
}
fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let width = input.lines().nth(0).unwrap().len();
    let mut occurrences = (0..width)
        .map(|_| HashMap::new())
        .collect::<Vec<HashMap<char, i32>>>();
    input.lines().for_each(|x| {
        x.chars().enumerate().for_each(|(i, c)| {
            let e = occurrences[i].entry(c).or_insert(0);
            *e += 1;
        })
    });
    Ok(occurrences
        .iter()
        .map(|x| *(x.iter().min_by_key(|(_, count)| *count).unwrap().0))
        .collect::<String>())
}
