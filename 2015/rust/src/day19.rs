use simple_error::require_with;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day19(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 19: Medicine for Rudolph");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn input_to_replacements(input: &str) -> Result<(HashMap<&str, Vec<&str>>, &str), Box<dyn Error>> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut map = HashMap::new();
    for line in require_with!(parts.get(0), "Invalid input").lines() {
        let kv = line.split(" => ").collect::<Vec<&str>>();
        map.entry(*require_with!(
            kv.get(0),
            &format!("Invalid input: {}", line)
        ))
        .or_insert(Vec::new())
        .push(*require_with!(
            kv.get(1),
            &format!("Invalid input: {}", line)
        ))
    }

    Ok((map, require_with!(parts.get(1), "Invalid input")))
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let (replacements, molecule) = input_to_replacements(input)?;
    let mut strings = HashSet::new();
    for (k, v) in replacements.iter() {
        let mut index = 0;
        while let Some(i) = &molecule[index..].find(k) {
            for replacement in v {
                let mut replaced = String::new();
                replaced.push_str(&molecule[..index + i]);
                replaced.push_str(replacement);
                replaced.push_str(&molecule[index + i + k.len()..]);
                strings.insert(replaced);
            }
            index = index + i + 1;
        }
    }

    Ok(strings.len() as i32)
}

fn tokenize(input: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut cursor = 0;
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            let token = &input[cursor..i];
            if token.len() > 0 {
                tokens.push(token);
                cursor = i;
            }
        }
    }
    tokens.push(&input[cursor..]);
    tokens
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let input = input_to_replacements(input)?;
    let tokens = tokenize(input.1);
    let (mut delimiters, mut commas) = (0, 0);
    for tok in tokens.iter() {
        if *tok == "Rn" || *tok == "Ar" {
            delimiters += 1;
        } else if *tok == "Y" {
            commas += 1;
        }
    }

    Ok((tokens.len() as i32) - delimiters - (2 * commas) - 1)
}
