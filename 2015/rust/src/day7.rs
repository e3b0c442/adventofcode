use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{bail, require_with, SimpleError};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day7(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 7: Some Assembly Required");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let sol1 = part1(&input)?;
    println!("\tPart 1: {}", sol1);
    println!("\tPart 2: {}", part2(&input, sol1 as u16)?);
    Ok(())
}

lazy_static! {
    static ref GATES_RE: Regex = Regex::new(r"(.*) -> ([a-z]+)").unwrap();
    static ref GATE_RE: Regex =
        Regex::new(r"(?:([a-z]+|[0-9]+) )?(?:(AND|OR|NOT|RSHIFT|LSHIFT) )?([a-z]+|[0-9]+)")
            .unwrap();
}

fn build_gates(input: &str) -> Result<HashMap<&str, &str>, SimpleError> {
    input.lines().try_fold(HashMap::new(), |mut map, line| {
        let caps = require_with!(GATES_RE.captures(line), &format!("Invalid input: {}", line));
        map.insert(
            require_with!(caps.get(2), &format!("Invalid input: {}", line)).as_str(),
            require_with!(caps.get(1), &format!("Invalid input: {}", line)).as_str(),
        );
        Ok(map)
    })
}

fn exec_gate(
    gates: &HashMap<&str, &str>,
    key: &str,
    cache: &mut HashMap<String, u16>,
) -> Result<u16, Box<dyn Error>> {
    if let Some(v) = cache.get(key) {
        return Ok(*v);
    }

    let gate = require_with!(gates.get(key), &format!("No gate for key {}", key));
    let caps = require_with!(GATE_RE.captures(gate), &format!("Invalid input: {}", gate));
    let l = match caps.get(1) {
        Some(cap) => match cap.as_str().parse::<u16>() {
            Ok(l) => l,
            Err(_) => exec_gate(gates, cap.as_str(), cache)?,
        },
        None => 0,
    };
    let r = match caps.get(3) {
        Some(cap) => match cap.as_str().parse::<u16>() {
            Ok(l) => l,
            Err(_) => exec_gate(gates, cap.as_str(), cache)?,
        },
        None => bail!(format!("Invalid input: {}", gate)),
    };
    let sol = match caps.get(2) {
        Some(cap) => match cap.as_str() {
            "LSHIFT" => l << r,
            "RSHIFT" => l >> r,
            "AND" => l & r,
            "OR" => l | r,
            "NOT" => !r,
            _ => bail!(format!("Invalid input: {}", gate)),
        },
        None => r,
    };
    cache.insert(key.to_string(), sol);
    Ok(sol)
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let gates = build_gates(input)?;
    let mut cache: HashMap<String, u16> = HashMap::new();
    match exec_gate(&gates, "a", &mut cache) {
        Ok(v) => Ok(v as i32),
        Err(e) => Err(e),
    }
}

fn part2(input: &str, start: u16) -> Result<i32, Box<dyn Error>> {
    let gates = build_gates(input)?;
    let mut cache: HashMap<String, u16> = HashMap::new();
    cache.insert("b".to_string(), start);
    match exec_gate(&gates, "a", &mut cache) {
        Ok(v) => Ok(v as i32),
        Err(e) => Err(e),
    }
}
