use lazy_static::lazy_static;
use regex::Regex;
use simple_error::require_with;
use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day15(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 15: Science for Hungry People");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

lazy_static! {
    static ref INGREDIENT_RE: Regex = Regex::new(r"([A-Za-z]+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
}

#[derive(Debug)]
struct Ingredient {
    scores: Vec<i32>,
    calories: i32,
}

fn input_to_ingredients(input: &str) -> Result<Vec<Ingredient>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let caps = require_with!(
                INGREDIENT_RE.captures(line),
                &format!("Invalid input: {}", line)
            );
            let ingrd = Ingredient {
                scores: vec![
                    require_with!(caps.get(2), &format!("Invalid input: {}", line))
                        .as_str()
                        .parse::<i32>()?,
                    require_with!(caps.get(3), &format!("Invalid input: {}", line))
                        .as_str()
                        .parse::<i32>()?,
                    require_with!(caps.get(4), &format!("Invalid input: {}", line))
                        .as_str()
                        .parse::<i32>()?,
                    require_with!(caps.get(5), &format!("Invalid input: {}", line))
                        .as_str()
                        .parse::<i32>()?,
                ],
                calories: require_with!(caps.get(6), &format!("Invalid input: {}", line))
                    .as_str()
                    .parse::<i32>()?,
            };
            Ok(ingrd)
        })
        .collect()
}

fn build_recipes(i: usize, ingrs: usize, base: Vec<usize>, remaining: usize) -> Vec<Vec<usize>> {
    if i == ingrs - 2 {
        return (0..=remaining)
            .map(|ix| {
                let mut recipe = base.clone();
                recipe[i] = ix;
                recipe[i + 1] = remaining - ix;
                recipe
            })
            .collect();
    }

    (0..=remaining).fold(Vec::new(), |mut all, ix| {
        let mut recipe = base.clone();
        recipe[i] = ix;
        all.extend(build_recipes(i + 1, ingrs, recipe, remaining - ix));
        all
    })
}

fn score_recipe(ingrs: &Vec<Ingredient>, recipe: &Vec<usize>) -> i32 {
    let scores_len = ingrs[0].scores.len();
    recipe
        .iter()
        .enumerate()
        .map(|(i, x)| {
            ingrs[i]
                .scores
                .iter()
                .map(|y| *y * (*x as i32))
                .collect::<Vec<i32>>()
        })
        .fold(vec![0; scores_len], |mut sums, ingr| {
            ingr.iter().enumerate().for_each(|(i, x)| {
                sums[i] += x;
            });
            sums
        })
        .into_iter()
        .map(|x| max(0, x))
        .product()
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let ingredients = input_to_ingredients(input)?;
    let mut scores = build_recipes(0, ingredients.len(), vec![0; ingredients.len()], 100)
        .into_iter()
        .map(|r| score_recipe(&ingredients, &r))
        .collect::<Vec<i32>>();
    scores.sort();
    Ok(*scores.last().unwrap())
}

fn get_calories(ingrs: &Vec<Ingredient>, recipe: &Vec<usize>) -> i32 {
    recipe
        .iter()
        .enumerate()
        .fold(0, |sum, (i, qty)| sum + ingrs[i].calories * (*qty as i32))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let ingredients = input_to_ingredients(input)?;
    let mut scores = build_recipes(0, ingredients.len(), vec![0; ingredients.len()], 100)
        .into_iter()
        .filter(|x| get_calories(&ingredients, x) == 500)
        .map(|r| score_recipe(&ingredients, &r))
        .collect::<Vec<i32>>();
    scores.sort();
    Ok(*scores.last().unwrap())
}
