use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day5(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 5: Doesn't He Have Intern-Elves For This?");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut nice = 0;
    'outer: for line in input.lines() {
        let mut vowels = 0;
        let mut last = '\0';
        let mut double = false;

        for c in line.chars() {
            match c {
                'b' => {
                    if last == 'a' {
                        continue 'outer;
                    }
                }
                'd' => {
                    if last == 'c' {
                        continue 'outer;
                    }
                }
                'q' => {
                    if last == 'p' {
                        continue 'outer;
                    }
                }
                'y' => {
                    if last == 'x' {
                        continue 'outer;
                    }
                }
                _ => (),
            };
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => (),
            };
            if c == last {
                double = true;
            }
            last = c;
        }
        if double == true && vowels >= 3 {
            nice += 1;
        }
    }
    Ok(nice)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut nice = 0;
    'outer: for line in input.lines() {
        let mut pairs: HashMap<String, Vec<usize>> = HashMap::new();
        let mut last = '\0';
        let mut two_ago = '\0';
        let (mut split, mut two_pair) = (false, false);

        for (i, c) in line.chars().enumerate() {
            let mut pair = String::new();
            pair.push(last);
            pair.push(c);
            let locs = pairs.entry(pair).or_insert(Vec::new());
            locs.push(i);

            if c == two_ago {
                split = true;
            }
            let locslen = locs.len();
            if (locslen > 1 && (locs[locslen - 1] - locs[locslen - 2] > 1)) || locslen > 2 {
                two_pair = true;
            }
            if split && two_pair {
                nice += 1;
                continue 'outer;
            }
            two_ago = last;
            last = c;
        }
    }
    Ok(nice)
}
