use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day12(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 12: JSAbacusFramework.io");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn json_sum(json: &Value) -> i32 {
    match json {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
        Value::Object(obj) => obj.values().fold(0, |sum, x| sum + json_sum(x)),
        Value::Array(arr) => arr.iter().fold(0, |sum, x| sum + json_sum(x)),
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(json_sum(&serde_json::from_str(input)?))
}

fn json_sum_no_red(json: &Value) -> i32 {
    match json {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
        Value::Object(obj) => match obj.values().any(|x| match x {
            Value::String(s) if s == "red" => true,
            _ => false,
        }) {
            true => 0,
            false => obj.values().fold(0, |sum, x| sum + json_sum_no_red(x)),
        },
        Value::Array(arr) => arr.iter().fold(0, |sum, x| sum + json_sum_no_red(x)),
    }
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(json_sum_no_red(&serde_json::from_str(input)?))
}
