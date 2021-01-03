use md5::{Digest, Md5};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day4(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 4: The Ideal Stocking Stuffer");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let start = Instant::now();
    let sol1 = part1(&input)?;
    println!("\tPart 1: {} ({:?})", sol1, start.elapsed());
    let second = Instant::now();
    println!(
        "\tPart 2: {} ({:?})",
        part2(&input, sol1)?,
        second.elapsed()
    );
    println!("\t\t Completed in {:?}", start.elapsed());
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut i = 1;
    let mut hasher = Md5::new();
    loop {
        let mut prefix = input.to_owned();
        prefix += &i.to_string();
        hasher.update(prefix.as_bytes());
        let hash = hasher.finalize_reset();

        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0 {
            return Ok(i);
        }
        i += 1;
    }
}

fn part2(input: &str, start: i32) -> Result<i32, Box<dyn Error>> {
    let mut i = start;
    let mut hasher = Md5::new();
    loop {
        let mut prefix = input.to_owned();
        prefix += &i.to_string();
        hasher.update(prefix.as_bytes());
        let hash = hasher.finalize_reset();

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return Ok(i);
        }
        i += 1;
    }
}
