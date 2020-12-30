use simple_error::{bail, require_with};
use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day22(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 22: Wizard Simulator 20XX");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Debug)]
struct SpellError {}

impl std::fmt::Display for SpellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spell cannot be cast")
    }
}

#[derive(Copy, Clone, Debug)]
struct Effects {
    shield: i32,
    poison: i32,
    recharge: i32,
}

#[derive(Copy, Clone, Debug)]
struct Combatant {
    hp: i32,
    damage: i32,
    armor: i32,
    mana: i32,
}

fn input_to_enemy(input: &str) -> Result<Combatant, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            Ok((
                *require_with! {parts.get(0), "Invalid input"},
                (*require_with! {parts.get(1), "Invalid input"}).parse::<i32>()?,
            ))
        })
        .collect::<Result<Vec<(&str, i32)>, Box<dyn Error>>>()?
        .iter()
        .try_fold(
            Combatant {
                hp: 0,
                armor: 0,
                damage: 0,
                mana: 0,
            },
            |mut e, a| {
                match a.0 {
                    "Hit Points" => e.hp = a.1,
                    "Damage" => e.damage = a.1,
                    "Armor" => e.armor = a.1,
                    _ => bail!("Invalid input"),
                };
                Ok(e)
            },
        )
}

fn resolve_effects(player: &mut Combatant, enemy: &mut Combatant, effects: &mut Effects) {
    if effects.shield == 1 {
        player.armor -= 7;
    }
    if effects.poison >= 1 {
        enemy.hp -= 3;
    }
    if effects.recharge >= 1 {
        player.mana += 101;
    }

    if effects.shield > 0 {
        effects.shield -= 1;
    }
    if effects.poison > 0 {
        effects.poison -= 1;
    }
    if effects.recharge > 0 {
        effects.recharge -= 1;
    }
}

type Spell = fn(&mut Combatant, &mut Combatant, &mut Effects) -> Option<i32>;

fn magic_missile(player: &mut Combatant, enemy: &mut Combatant, _: &mut Effects) -> Option<i32> {
    player.mana -= 53;
    if player.mana < 0 {
        return None;
    }
    enemy.hp -= 4;
    Some(53)
}

fn drain(player: &mut Combatant, enemy: &mut Combatant, _: &mut Effects) -> Option<i32> {
    player.mana -= 73;
    if player.mana < 0 {
        return None;
    }
    enemy.hp -= 2;
    player.hp += 2;
    Some(73)
}

fn shield(player: &mut Combatant, _: &mut Combatant, effects: &mut Effects) -> Option<i32> {
    if effects.shield > 0 {
        return None;
    }
    player.mana -= 113;
    if player.mana < 0 {
        return None;
    }
    player.armor += 7;
    effects.shield = 6;
    Some(113)
}

fn poison(player: &mut Combatant, _: &mut Combatant, effects: &mut Effects) -> Option<i32> {
    if effects.poison > 0 {
        return None;
    }
    player.mana -= 173;
    if player.mana < 0 {
        return None;
    }
    effects.poison = 6;
    Some(173)
}

fn recharge(player: &mut Combatant, _: &mut Combatant, effects: &mut Effects) -> Option<i32> {
    if effects.recharge > 0 {
        return None;
    }
    player.mana -= 229;
    if player.mana < 0 {
        return None;
    }
    effects.recharge = 5;
    Some(229)
}

/* old fn player_turn(
    mut player: Combatant,
    mut enemy: Combatant,
    mut effects: Effects,
    spells: &Vec<Spell>,
    drain: i32,
    spent: i32,
    depth: i32,
) -> Option<i32> {
    player.hp -= drain;
    if player.hp <= 0 {
        return None;
    }
    resolve_effects(&mut player, &mut enemy, &mut effects);
    if enemy.hp <= 0 {
        return Some(spent);
    }
    for spell in spells.iter() {
        let mut player = player;
        let mut enemy = enemy;
        let mut effects = effects;
        match spell(&mut player, &mut enemy, &mut effects) {
            None => (),
            Some(x) => {
                if enemy.hp <= 0 {
                    return Some(spent + x);
                }
                match enemy_turn(player, enemy, effects, spells, drain, spent + x, depth + 1) {
                    Some(x) => return Some(x),
                    None => (),
                }
            }
        }
    }
    None
}*/

fn player_turn(
    mut player: Combatant,
    mut enemy: Combatant,
    mut effects: Effects,
    spells: &Vec<Spell>,
    unordered: bool,
    drain: i32,
    spent: i32,
    depth: i32,
) -> Option<i32> {
    player.hp -= drain;
    if player.hp <= 0 {
        return None;
    }
    resolve_effects(&mut player, &mut enemy, &mut effects);
    if enemy.hp <= 0 {
        return Some(spent);
    }

    let mut results = Vec::new();
    for spell in spells.iter() {
        let mut player = player;
        let mut enemy = enemy;
        let mut effects = effects;
        match spell(&mut player, &mut enemy, &mut effects) {
            None => (),
            Some(x) => {
                if enemy.hp <= 0 {
                    if unordered {
                        results.push(spent + x);
                    } else {
                        return Some(spent + x);
                    }
                } else {
                    match enemy_turn(player, enemy, effects, spells, drain, spent + x, depth + 1) {
                        Some(x) => {
                            if unordered {
                                results.push(x);
                            } else {
                                return Some(x);
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    }
    results.into_iter().min()
}

fn enemy_turn(
    mut player: Combatant,
    mut enemy: Combatant,
    mut effects: Effects,
    spells: &Vec<Spell>,
    drain: i32,
    spent: i32,
    depth: i32,
) -> Option<i32> {
    resolve_effects(&mut player, &mut enemy, &mut effects);
    if enemy.hp <= 0 {
        return Some(spent);
    }
    player.hp -= max(enemy.damage - player.armor, 1);
    if player.hp <= 0 {
        return None;
    }
    player_turn(
        player,
        enemy,
        effects,
        spells,
        true,
        drain,
        spent,
        depth + 1,
    )
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let enemy = input_to_enemy(input)?;
    let player = Combatant {
        hp: 50,
        armor: 0,
        damage: 0,
        mana: 500,
    };
    let effects = Effects {
        shield: 0,
        poison: 0,
        recharge: 0,
    };
    let spells: Vec<Spell> = vec![poison, magic_missile, drain, shield, recharge];

    match player_turn(player, enemy, effects, &spells, false, 0, 0, 1) {
        Some(x) => Ok(x),
        None => bail!("I can't win"),
    }
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let enemy = input_to_enemy(input)?;
    let player = Combatant {
        hp: 50,
        armor: 0,
        damage: 0,
        mana: 500,
    };
    let effects = Effects {
        shield: 0,
        poison: 0,
        recharge: 0,
    };
    let spells: Vec<Spell> = vec![poison, magic_missile, shield, drain, recharge];

    match player_turn(player, enemy, effects, &spells, true, 1, 0, 1) {
        Some(x) => Ok(x),
        None => bail!("I can't win"),
    }
}
