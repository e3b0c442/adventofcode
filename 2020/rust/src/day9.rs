use simple_error::SimpleError;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day9(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 9: Encoding Error");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let p1 = part1(&input)?;
    println!("\tPart 1: {}", p1);
    println!("\tPart 2: {}", part2(&input, p1)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut seed = VecDeque::new();
    let mut sums: HashSet<i32> = HashSet::new();
    for line in input.lines() {
        let v = line.parse::<i32>()?;
        if seed.len() == 25 {
            if !sums.contains(&v) {
                return Ok(v);
            }
        }

        seed.push_back(v);
        if seed.len() < 25 {
            continue;
        }
        if seed.len() > 25 {
            seed.pop_front();
        }
        sums.clear();
        for i in 0..seed.len() {
            for j in i..seed.len() {
                sums.insert(seed[i] + seed[j]);
            }
        }
    }
    Err(Box::new(SimpleError::new("Solution not found")))
}

fn part2(input: &str, notsum: i32) -> Result<i32, Box<dyn Error>> {
    let nums: Vec<i32> = input
        .lines()
        .rev()
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    for l in 0..nums.len() {
        for r in l + 1..nums.len() {
            let mut sl = nums[l..r + 1].to_vec();
            let sum = sl.iter().fold(0, |s, x| s + *x as i64);
            if sum == notsum as i64 {
                sl.sort();
                return Ok(sl[0] + sl[sl.len() - 1]);
            }
            if sum > notsum as i64 {
                break;
            }
        }
    }
    Ok(0)
}
