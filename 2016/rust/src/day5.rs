use md5::{Digest, Md5};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day5(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 5: How About a Nice Game of Chess?");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut i = 0;
    let mut hasher = Md5::new();
    let mut password = String::new();
    while password.len() < 8 {
        let mut prefix = input.to_owned();
        prefix.push_str(&i.to_string());
        hasher.update(prefix.as_bytes());
        let hash = hasher.finalize_reset();
        i += 1;
        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0 {
            password += &format!("{:x}", hash)[5..6];
        }
    }
    Ok(password)
}

fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut i = 0;
    let mut hasher = Md5::new();
    let mut found = 0;
    let mut password: [Option<char>; 8] = [None; 8];
    while found < 8 {
        let mut prefix = input.to_owned();
        prefix.push_str(&i.to_string());
        hasher.update(prefix.as_bytes());
        let hash = hasher.finalize_reset();
        i += 1;
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 8 {
            match password[hash[2] as usize] {
                Some(_) => (),
                None => {
                    password[hash[2] as usize] = (&format!("{:x}", hash)[6..7]).chars().next();
                    found += 1;
                }
            }
        }
    }
    Ok(password.iter().flatten().collect::<String>())
}
