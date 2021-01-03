use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day11(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 11: Corporate Policy");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let start = Instant::now();
    let sol1 = part1(&input)?;
    println!("\tPart 1: {} ({:?})", sol1, start.elapsed());
    let second = Instant::now();
    println!("\tPart 2: {} ({:?})", part2(&sol1)?, second.elapsed());
    println!("\t\t Completed in {:?}", start.elapsed());
    Ok(())
}

fn increment(input: &mut Vec<u8>) {
    for c in input.into_iter().rev() {
        if char::from(*c) == 'z' {
            let mut b: [u8; 1] = [0];
            'a'.encode_utf8(&mut b[..]);
            *c = b[0];
        } else {
            *c += 1;
            return;
        }
    }
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut input: Vec<u8> = Vec::from(input.as_bytes());
    loop {
        let (mut one_ago, mut two_ago, mut three_ago) = (0, 0, 0);
        let (mut pairs, mut triplet) = (0, false);
        for uc in input.iter() {
            let c = char::from(*uc);
            match c {
                'i' | 'o' | 'l' => break,
                _ => (),
            }
            if *uc as i32 - one_ago as i32 == 1 && one_ago as i32 - two_ago as i32 == 1 {
                triplet = true;
            }
            if *uc == one_ago && (*uc != two_ago || (two_ago == three_ago)) {
                pairs += 1;
            }
            if triplet && pairs >= 2 {
                return Ok(String::from_utf8(input)?);
            }
            three_ago = two_ago;
            two_ago = one_ago;
            one_ago = *uc;
        }
        increment(&mut input);
    }
}

fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut input: Vec<u8> = Vec::from(input.as_bytes());
    increment(&mut input);
    loop {
        let (mut one_ago, mut two_ago, mut three_ago) = (0, 0, 0);
        let (mut pairs, mut triplet) = (0, false);
        for uc in input.iter() {
            let c = char::from(*uc);
            match c {
                'i' | 'o' | 'l' => break,
                _ => (),
            }
            if *uc as i32 - one_ago as i32 == 1 && one_ago as i32 - two_ago as i32 == 1 {
                triplet = true;
            }
            if *uc == one_ago && (*uc != two_ago || (two_ago == three_ago)) {
                pairs += 1;
            }
            if triplet && pairs >= 2 {
                return Ok(String::from_utf8(input)?);
            }
            three_ago = two_ago;
            two_ago = one_ago;
            one_ago = *uc;
        }
        increment(&mut input);
    }
}
