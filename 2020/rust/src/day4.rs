use simple_error::SimpleError;
use std::collections::HashMap;
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
    let mut passports = Vec::new();
    let records = input.split("\n\n");
    for record in records {
        let mut passport: HashMap<String, String> = HashMap::new();
        let record = record.replace("\n", " ");
        let fields = record.split(" ");
        for field in fields {
            let kv: Vec<&str> = field.split(":").collect();
            if let Some(_) = passport.insert(kv[0].to_string(), kv[1].to_string()) {
                return Err(Box::new(SimpleError::new("duplicate field in passport")));
            }
        }
        passports.push(passport)
    }

    return Ok(passports);
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let passports = input_to_passports(input)?;
    let mut valid = 0;
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    'pp: for passport in passports {
        for field in fields.iter() {
            if let None = passport.get(&field.to_string()) {
                continue 'pp;
            }
        }
        valid += 1;
    }
    Ok(valid)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let passports = input_to_passports(input)?;
    let mut valid = 0;

    for passport in passports {
        match passport.get("byr") {
            None => continue,
            Some(byr) => {
                if byr.len() != 4 {
                    continue;
                }
                match byr.parse::<i32>() {
                    Err(_) => continue,
                    Ok(byr) => {
                        if byr < 1920 || byr > 2002 {
                            continue;
                        }
                    }
                }
            }
        };
        match passport.get("iyr") {
            None => continue,
            Some(iyr) => {
                if iyr.len() != 4 {
                    continue;
                }
                match iyr.parse::<i32>() {
                    Err(_) => continue,
                    Ok(iyr) => {
                        if iyr < 2010 || iyr > 2020 {
                            continue;
                        }
                    }
                }
            }
        };
        match passport.get("eyr") {
            None => continue,
            Some(eyr) => {
                if eyr.len() != 4 {
                    continue;
                }
                match eyr.parse::<i32>() {
                    Err(_) => continue,
                    Ok(eyr) => {
                        if eyr < 2020 || eyr > 2030 {
                            continue;
                        }
                    }
                }
            }
        };
        match passport.get("hgt") {
            None => continue,
            Some(hgt) => {
                let unit = &hgt[hgt.len() - 2..];
                let value = &hgt[0..hgt.len() - 2];
                match value.parse::<i32>() {
                    Err(_) => continue,
                    Ok(value) => match unit {
                        "cm" => {
                            if value < 150 || value > 193 {
                                continue;
                            }
                        }
                        "in" => {
                            if value < 59 || value > 76 {
                                continue;
                            }
                        }
                        _ => continue,
                    },
                }
            }
        };
        match passport.get("hcl") {
            None => continue,
            Some(hcl) => {
                if hcl[0..1].ne("#") {
                    continue;
                }
                match i32::from_str_radix(&hcl[1..], 16) {
                    Err(_) => continue,
                    Ok(_) => {}
                };
            }
        }
        match passport.get("ecl") {
            None => continue,
            Some(ecl) => match &ecl[..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => continue,
            },
        }
        match passport.get("pid") {
            None => continue,
            Some(pid) => {
                if pid.len() != 9 {
                    continue;
                }
                match pid.parse::<i32>() {
                    Err(_) => continue,
                    Ok(_) => {}
                }
            }
        }

        valid += 1;
    }

    Ok(valid)
}
