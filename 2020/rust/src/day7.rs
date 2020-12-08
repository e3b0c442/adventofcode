use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"^([a-z]+ [a-z]+) bags contain (.*)\.$").unwrap();
    static ref BAGGED_RE: Regex = Regex::new(r"(\d)+ ([a-z]+ [a-z]+) bags?").unwrap();
}

pub fn day7(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 7: Handy Haversacks");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn find_shiny_gold(bags: &HashMap<&str, Vec<&str>>, trail: Vec<&str>) -> bool {
    if bags
        .get(trail.iter().last().unwrap())
        .unwrap()
        .iter()
        .any(|&x| x == "shiny gold")
    {
        return true;
    }
    for bag in bags.get(trail.last().unwrap()).unwrap() {
        if trail.iter().any(|x| x == bag) {
            continue;
        }
        let mut new_trail = trail.clone();
        new_trail.push(bag);
        if find_shiny_gold(bags, new_trail) {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut bags: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let bag_match = BAG_RE.captures(line).unwrap();
        let (k, v) = (
            bag_match.get(1).unwrap().as_str(),
            bag_match.get(2).unwrap().as_str(),
        );
        bags.insert(
            k,
            BAGGED_RE
                .captures_iter(v)
                .map(|x| x.get(2).unwrap().as_str())
                .collect(),
        );
    }
    Ok(bags
        .iter()
        .map(|(k, _)| find_shiny_gold(&bags, vec![k]))
        .fold(0, |sum, x| if x { sum + 1 } else { sum }))
}

fn count_in(bags: &HashMap<&str, HashMap<&str, i32>>, key: &str) -> i32 {
    bags.get(key)
        .unwrap()
        .iter()
        .fold(0, |sum, (k, v)| sum + v + v * count_in(bags, k))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut bags: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in input.lines() {
        let bag_match = BAG_RE.captures(line).unwrap();
        let (k, v) = (
            bag_match.get(1).unwrap().as_str(),
            bag_match.get(2).unwrap().as_str(),
        );
        bags.insert(
            k,
            BAGGED_RE
                .captures_iter(v)
                .fold(HashMap::new(), |mut map, x| {
                    map.insert(
                        x.get(2).unwrap().as_str(),
                        x.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    );
                    map
                }),
        );
    }
    Ok(count_in(&bags, "shiny gold"))
}
