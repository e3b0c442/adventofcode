use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{bail, require_with};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day4(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 4: Security Through Obscurity");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

lazy_static! {
    static ref ROOM_RE: Regex = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();
}

struct Room<'a> {
    name: &'a str,
    sector: i32,
    checksum: &'a str,
}

fn line_to_room(line: &str) -> Result<Room, Box<dyn Error>> {
    let caps = require_with!(ROOM_RE.captures(line), &format!("Invalid input: {}", line));

    Ok(Room {
        name: require_with!(caps.get(1), &format!("Invalid input: {}", line)).as_str(),
        sector: require_with!(caps.get(2), &format!("Invalid input: {}", line))
            .as_str()
            .parse::<i32>()?,
        checksum: require_with!(caps.get(3), &format!("Invalid input: {}", line)).as_str(),
    })
}

fn checksum(name: &str) -> String {
    let mut map = HashMap::new();
    let clean_name = name.replace("-", "");
    for c in clean_name.chars() {
        let e = map.entry(c).or_insert(0);
        *e += 1;
    }
    let mut sorted = map.into_iter().collect::<Vec<(char, i32)>>();
    sorted.sort_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        x @ _ => x,
    });
    (&sorted.into_iter().map(|x| x.0).collect::<String>()[0..5]).to_owned()
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|x| line_to_room(x))
        .collect::<Result<Vec<Room>, _>>()?
        .iter()
        .fold(0, |sum, room| match checksum(room.name) == room.checksum {
            true => sum + room.sector,
            false => sum,
        }))
}

fn decrypt(name: &str, rotation: i32) -> String {
    name.chars()
        .map(|c| match c {
            '-' => ' ',
            _ => {
                let advance = rotation % 26;
                let mut position = c as i32 - 'a' as i32;
                position += advance;
                if position >= 26 {
                    position -= 26;
                }
                (position + 'a' as i32) as u8 as char
            }
        })
        .collect::<String>()
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(
        match input
            .lines()
            .map(|x| line_to_room(x))
            .collect::<Result<Vec<Room>, _>>()?
            .iter()
            .find(|room| decrypt(room.name, room.sector) == "northpole object storage")
        {
            Some(x) => x.sector,
            None => bail!("Solution not found"),
        },
    )
}
