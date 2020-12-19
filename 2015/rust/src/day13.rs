use lazy_static::lazy_static;
use regex::Regex;
use simple_error::require_with;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day13(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 13: Knights of the Dinner Table");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

lazy_static! {
    static ref SEATING_RE: Regex = Regex::new(
        r"([A-Za-z]+) would (gain|lose) (\d+) happiness units by sitting next to ([A-Za-z]+)."
    )
    .unwrap();
}

fn seating_chart(input: &str) -> Result<HashMap<&str, HashMap<&str, i32>>, Box<dyn Error>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let caps = require_with!(
            SEATING_RE.captures(line),
            &format!("Invalid input: {}", line)
        );
        let (person_1, adj, val, person_2) = (
            require_with!(caps.get(1), &format!("Invalid input: {}", line)).as_str(),
            require_with!(caps.get(2), &format!("Invalid input: {}", line)).as_str(),
            require_with!(caps.get(3), &format!("Invalid input: {}", line))
                .as_str()
                .parse::<i32>()?,
            require_with!(caps.get(4), &format!("Invalid input: {}", line)).as_str(),
        );
        let val = match adj {
            "gain" => val,
            "lose" => -val,
            _ => val,
        };
        let inner = map.entry(person_1).or_insert(HashMap::new());
        inner.insert(person_2, val);
    }
    Ok(map)
}

fn happiness(
    map: &HashMap<&str, HashMap<&str, i32>>,
    people: Vec<&str>,
    remaining: HashSet<&str>,
    total: i32,
) -> Vec<i32> {
    if remaining.len() == 0 {
        return vec![
            total
                + *map
                    .get(people.first().unwrap())
                    .unwrap()
                    .get(people.last().unwrap())
                    .unwrap()
                + *map
                    .get(people.last().unwrap())
                    .unwrap()
                    .get(people.first().unwrap())
                    .unwrap(),
        ];
    }

    let mut result = Vec::new();
    for person in remaining.iter() {
        let mut remaining = remaining.clone();
        let mut people = people.clone();
        let total = total
            + *map
                .get(people.last().unwrap())
                .unwrap()
                .get(person)
                .unwrap()
            + *map
                .get(person)
                .unwrap()
                .get(people.last().unwrap())
                .unwrap();

        remaining.remove(person);
        people.push(person);
        result.extend(happiness(map, people, remaining, total))
    }
    result
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let seating = seating_chart(input)?;
    let people: HashSet<&str> = seating.keys().map(|k| *k).collect();

    let mut happy = people.iter().fold(Vec::new(), |mut all, person| {
        let mut remaining = people.clone();
        remaining.remove(person);
        all.extend(happiness(&seating, vec![person], remaining, 0));
        all
    });
    happy.sort();

    Ok(*require_with!(happy.last(), ""))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let orig_seating = seating_chart(input)?;
    let mut me: HashMap<&str, i32> = HashMap::new();

    let mut people: HashSet<&str> = orig_seating.keys().map(|k| *k).collect();
    let mut seating = HashMap::new();
    for (key, entry) in orig_seating {
        me.insert(key, 0);
        let mut new_entry = entry.clone();
        new_entry.insert("me", 0);
        seating.insert(key, new_entry);
    }
    seating.insert("me", me);
    people.insert("me");

    let mut happy = people.iter().fold(Vec::new(), |mut all, person| {
        let mut remaining = people.clone();
        remaining.remove(person);
        all.extend(happiness(&seating, vec![person], remaining, 0));
        all
    });
    happy.sort();

    Ok(*require_with!(happy.last(), ""))
}
