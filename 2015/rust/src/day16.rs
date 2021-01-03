use simple_error::{bail, require_with};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub fn day16(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 16: Aunt Sue");

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

#[derive(Debug)]
struct Sue<'a> {
    number: i32,
    fields: HashMap<&'a str, i32>,
}

fn input_to_sues(input: &str) -> Result<Vec<Sue>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let parts = line.splitn(2, ": ").collect::<Vec<&str>>();
            let sue = require_with!(
                require_with!(parts.get(0), &format!("Invalid input: {}", line))
                    .strip_prefix("Sue "),
                &format!("Invalid input: {}", line)
            )
            .parse::<i32>()?;

            let fields = require_with!(parts.get(1), &format!("Invalid input: {}", line))
                .split(", ")
                .try_fold::<_, _, Result<HashMap<&str, i32>, Box<dyn Error>>>(
                    HashMap::new(),
                    |mut map, kva| {
                        let kv = kva.split(": ").collect::<Vec<&str>>();
                        map.insert(
                            *require_with!(kv.get(0), &format!("Invalid input: {}", line)),
                            require_with!(kv.get(1), &format!("Invalid input: {}", line))
                                .parse::<i32>()?,
                        );
                        Ok(map)
                    },
                )?;

            Ok(Sue {
                number: sue,
                fields: fields,
            })
        })
        .collect()
}

fn filter_predicate(fields: &HashMap<&str, i32>, predicate: &[(&str, (&str, i32))]) -> bool {
    match predicate
        .iter()
        .try_fold((), |_, (k, (op, v))| match fields.get(k) {
            None => Ok(()),
            Some(val) => match *op {
                "eq" => {
                    if val == v {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                "gt" => {
                    if val > v {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                "lt" => {
                    if val < v {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            },
        }) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let sues = input_to_sues(input)?;
    let predicate = [
        ("children", ("eq", 3)),
        ("cats", ("eq", 7)),
        ("samoyeds", ("eq", 2)),
        ("pomeranians", ("eq", 3)),
        ("akitas", ("eq", 0)),
        ("vizslas", ("eq", 0)),
        ("goldfish", ("eq", 5)),
        ("trees", ("eq", 3)),
        ("cars", ("eq", 2)),
        ("perfumes", ("eq", 1)),
    ];

    let filtered = sues
        .into_iter()
        .filter(|x| filter_predicate(&x.fields, &predicate))
        .collect::<Vec<Sue>>();

    if filtered.len() != 1 {
        bail!("Expected 1 result, got {}", filtered.len());
    }

    Ok(filtered[0].number)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let sues = input_to_sues(input)?;
    let predicate = [
        ("children", ("eq", 3)),
        ("cats", ("gt", 7)),
        ("samoyeds", ("eq", 2)),
        ("pomeranians", ("lt", 3)),
        ("akitas", ("eq", 0)),
        ("vizslas", ("eq", 0)),
        ("goldfish", ("lt", 5)),
        ("trees", ("gt", 3)),
        ("cars", ("eq", 2)),
        ("perfumes", ("eq", 1)),
    ];

    let filtered = sues
        .into_iter()
        .filter(|x| filter_predicate(&x.fields, &predicate))
        .collect::<Vec<Sue>>();

    if filtered.len() != 1 {
        bail!("Expected 1 result, got {}", filtered.len());
    }

    Ok(filtered[0].number)
}
