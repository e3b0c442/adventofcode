use lazy_static::lazy_static;
use regex::Regex;
use simple_error::require_with;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day9(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 9: All in a Single Night");

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
    static ref DIST_RE: Regex = Regex::new(r"([A-Za-z]+) to ([A-Za-z]+) = (\d+)").unwrap();
}

fn input_to_map(input: &str) -> Result<HashMap<&str, HashMap<&str, i32>>, Box<dyn Error>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let caps = require_with!(DIST_RE.captures(line), &format!("Invalid input: {}", line));
        let (orig, dest, dist) = (
            require_with!(caps.get(1), &format!("Invalid input: {}", line)).as_str(),
            require_with!(caps.get(2), &format!("Invalid input: {}", line)).as_str(),
            require_with!(caps.get(3), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<i32>()?,
        );

        let from = map.entry(orig).or_insert(HashMap::new());
        from.insert(dest, dist);
        let to = map.entry(dest).or_insert(HashMap::new());
        to.insert(orig, dist);
    }

    return Ok(map);
}

fn find_path_lens(
    dists: &HashMap<&str, HashMap<&str, i32>>,
    origin: &str,
    remaining: HashSet<&str>,
    so_far: i32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    if remaining.len() == 0 {
        return Ok(vec![so_far]);
    }

    let mut result = vec![];
    for dest in remaining.iter() {
        let mut clone = remaining.clone();
        clone.remove(dest);

        let leg = *require_with!(
            require_with!(
                dists.get(origin),
                &format!("{} to {} not found", origin, dest)
            )
            .get(dest),
            &format!("{} to {} not fonud", origin, dest)
        );

        result.extend(find_path_lens(dists, dest, clone, so_far + leg)?);
    }
    Ok(result)
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let map = input_to_map(input)?;
    let cities = map.keys().map(|k| *k).collect::<HashSet<&str>>();
    let mut paths = cities
        .iter()
        .try_fold::<_, _, Result<Vec<i32>, Box<dyn Error>>>(vec![], |mut sum, x| {
            let mut clone = cities.clone();
            clone.remove(x);
            sum.extend(find_path_lens(&map, x, clone, 0)?);
            Ok(sum)
        })?;
    paths.sort();

    Ok(*paths.first().unwrap())
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let map = input_to_map(input)?;
    let cities = map.keys().map(|k| *k).collect::<HashSet<&str>>();
    let mut paths = cities
        .iter()
        .try_fold::<_, _, Result<Vec<i32>, Box<dyn Error>>>(vec![], |mut sum, x| {
            let mut clone = cities.clone();
            clone.remove(x);
            sum.extend(find_path_lens(&map, x, clone, 0)?);
            Ok(sum)
        })?;
    paths.sort();

    Ok(*paths.last().unwrap())
}
