use lazy_static::lazy_static;
use regex::Regex;
use simple_error::require_with;
use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day14(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 14: Reindeer Olympics");

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

lazy_static! {
    static ref REINDEER_RE: Regex = Regex::new(
        r"([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds."
    )
    .unwrap();
}

#[derive(Debug)]
struct Reindeer {
    velocity: i32,
    fly: i32,
    rest: i32,
}

fn parse_reindeer(input: &str) -> Result<HashMap<&str, Reindeer>, Box<dyn Error>> {
    let mut reindeer = HashMap::new();
    for line in input.lines() {
        let caps = require_with!(
            REINDEER_RE.captures(line),
            &format!("Invalid input: {}", line)
        );
        let name = require_with!(caps.get(1), &format!("Invalid input: {}", line)).as_str();
        let deer = Reindeer {
            velocity: require_with!(caps.get(2), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<i32>()?,
            fly: require_with!(caps.get(3), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<i32>()?,
            rest: require_with!(caps.get(4), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<i32>()?,
        };
        reindeer.insert(name, deer);
    }
    Ok(reindeer)
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let reindeer = parse_reindeer(input)?;
    //println!("{:?}", reindeer);
    Ok(reindeer
        .values()
        .map(|r| {
            (2503 / (r.fly + r.rest)) * (r.velocity * r.fly)
                + min(2503 % (r.fly + r.rest), r.fly) * r.velocity
        })
        .max()
        .unwrap())
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let reindeer = parse_reindeer(input)?;
    let mut scores: HashMap<&str, i32> = HashMap::new();

    for i in 1..=2503 {
        let mut max = 0;
        let mut winners = vec![];
        for (n, r) in &reindeer {
            let score = (i / (r.fly + r.rest)) * (r.velocity * r.fly)
                + min(i % (r.fly + r.rest), r.fly) * r.velocity;
            if score > max {
                winners.clear();
                winners.push(n);
                max = score;
            } else if score == max {
                winners.push(n);
            }
        }

        for n in winners {
            let score = scores.entry(n).or_insert(0);
            *score += 1;
        }
    }
    Ok(scores.values().map(|x| *x).max().unwrap())
}
