use simple_error::SimpleError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day4(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 4: Passport Processing");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn input_to_passports(input: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    input
        .split("\n\n")
        .try_fold(Vec::new(), |mut passports, record| {
            let passport = record.replace("\n", " ").split(" ").try_fold(
                HashMap::new(),
                |mut passport, field| -> Result<HashMap<String, String>, Box<dyn Error>> {
                    let kv: Vec<&str> = field.split(":").collect();
                    if kv.len() < 2 {
                        return Err(SimpleError::new(format!("Invalid input: {}", record)).into());
                    }
                    passport.insert(kv[0].to_string(), kv[1].to_string());
                    Ok(passport)
                },
            )?;
            passports.push(passport);
            Ok(passports)
        })
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    Ok(input_to_passports(input)?
        .iter()
        .fold(0, |valid, passport| {
            let pfields: HashSet<&str> = passport.keys().map(|k| k.as_str()).collect();
            if pfields.intersection(&fields).count() == fields.len() {
                valid + 1
            } else {
                valid
            }
        }))
}

fn passport_valid(passport: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    match passport.get("byr") {
        None => return Err(SimpleError::new("").into()),
        Some(byr) => {
            if byr.len() != 4 {
                return Err(SimpleError::new("").into());
            }
            let byr = byr.parse::<i32>()?;
            if byr < 1920 || byr > 2002 {
                return Err(SimpleError::new("").into());
            }
        }
    };
    match passport.get("iyr") {
        None => return Err(SimpleError::new("").into()),
        Some(iyr) => {
            if iyr.len() != 4 {
                return Err(SimpleError::new("").into());
            }
            let iyr = iyr.parse::<i32>()?;
            if iyr < 2010 || iyr > 2020 {
                return Err(SimpleError::new("").into());
            }
        }
    };
    match passport.get("eyr") {
        None => return Err(SimpleError::new("").into()),
        Some(eyr) => {
            if eyr.len() != 4 {
                return Err(SimpleError::new("").into());
            }
            let eyr = eyr.parse::<i32>()?;
            if eyr < 2020 || eyr > 2030 {
                return Err(SimpleError::new("").into());
            }
        }
    };
    match passport.get("hgt") {
        None => return Err(SimpleError::new("").into()),
        Some(hgt) => {
            let unit = &hgt[hgt.len() - 2..];
            let value = hgt[0..hgt.len() - 2].parse::<i32>()?;
            match unit {
                "cm" => {
                    if value < 150 || value > 193 {
                        return Err(SimpleError::new("").into());
                    }
                }
                "in" => {
                    if value < 59 || value > 76 {
                        return Err(SimpleError::new("").into());
                    }
                }
                _ => return Err(SimpleError::new("").into()),
            }
        }
    };
    match passport.get("hcl") {
        None => return Err(SimpleError::new("").into()),
        Some(hcl) => {
            if hcl[0..1].ne("#") {
                return Err(SimpleError::new("").into());
            }
            i32::from_str_radix(&hcl[1..], 16)?;
        }
    }
    match passport.get("ecl") {
        None => return Err(SimpleError::new("").into()),
        Some(ecl) => match &ecl[..] {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => return Err(SimpleError::new("").into()),
        },
    }
    match passport.get("pid") {
        None => return Err(SimpleError::new("").into()),
        Some(pid) => {
            if pid.len() != 9 {
                return Err(SimpleError::new("").into());
            }
            pid.parse::<i32>()?;
        }
    }
    Ok(())
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    Ok(input_to_passports(input)?
        .iter()
        .fold(0, |valid, passport| match passport_valid(passport) {
            Ok(_) => valid + 1,
            Err(_) => valid,
        }))
}
