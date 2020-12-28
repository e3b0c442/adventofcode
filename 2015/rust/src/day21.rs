use lazy_static::lazy_static;
use simple_error::{bail, require_with};
use std::cmp::{max, Ordering};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn day21(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Day 21: RPG Simulator 20XX");

    let mut f = File::open(input_file)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("\tPart 1: {}", part1(&input)?);
    println!("\tPart 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

struct Store {
    weapons: [Item; 5],
    armors: [Item; 5],
    rings: [Item; 6],
}

#[derive(Copy, Clone, Debug)]
struct Combatant {
    hp: i32,
    damage: i32,
    armor: i32,
}

trait Equipment {
    fn cost(&self) -> i32;
    fn damage(&self) -> i32;
    fn armor(&self) -> i32;
}

impl Equipment for Vec<Item> {
    fn cost(&self) -> i32 {
        self.iter().map(|x| x.cost).sum()
    }
    fn damage(&self) -> i32 {
        self.iter().map(|x| x.damage).sum()
    }
    fn armor(&self) -> i32 {
        self.iter().map(|x| x.armor).sum()
    }
}

lazy_static! {
    static ref STORE: Store = Store {
        weapons: [
            Item {
                cost: 8,
                damage: 4,
                armor: 0
            },
            Item {
                cost: 10,
                damage: 5,
                armor: 0
            },
            Item {
                cost: 25,
                damage: 6,
                armor: 0
            },
            Item {
                cost: 40,
                damage: 7,
                armor: 0
            },
            Item {
                cost: 74,
                damage: 8,
                armor: 0
            }
        ],
        armors: [
            Item {
                cost: 13,
                damage: 0,
                armor: 1
            },
            Item {
                cost: 31,
                damage: 0,
                armor: 2
            },
            Item {
                cost: 53,
                damage: 0,
                armor: 3
            },
            Item {
                cost: 75,
                damage: 0,
                armor: 4
            },
            Item {
                cost: 102,
                damage: 0,
                armor: 5
            }
        ],
        rings: [
            Item {
                cost: 25,
                damage: 1,
                armor: 0
            },
            Item {
                cost: 50,
                damage: 2,
                armor: 0
            },
            Item {
                cost: 100,
                damage: 3,
                armor: 0
            },
            Item {
                cost: 20,
                damage: 0,
                armor: 1
            },
            Item {
                cost: 40,
                damage: 0,
                armor: 2
            },
            Item {
                cost: 80,
                damage: 0,
                armor: 3
            },
        ]
    };
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

fn build_equipment() -> Vec<Vec<Item>> {
    let mut sets = Vec::new();
    for weapon in STORE.weapons.iter() {
        sets.push(vec![*weapon]);
        for armor in STORE.armors.iter() {
            sets.push(vec![*weapon, *armor]);
            for i in 0..STORE.rings.len() {
                sets.push(vec![*weapon, *armor, STORE.rings[i]]);
                for j in i + 1..STORE.rings.len() {
                    sets.push(vec![*weapon, *armor, STORE.rings[i], STORE.rings[j]]);
                }
            }
        }
        for i in 0..STORE.rings.len() {
            sets.push(vec![*weapon, STORE.rings[i]]);
            for j in i + 1..STORE.rings.len() {
                sets.push(vec![*weapon, STORE.rings[i], STORE.rings[j]]);
            }
        }
    }

    sets
}

enum Winner {
    Player,
    Enemy,
}

fn do_battle(player: &Combatant, enemy: &Combatant) -> Winner {
    let me_turns = enemy.hp / max(player.damage - enemy.armor, 1)
        + match (enemy.hp / max(player.damage - enemy.armor, 1)).cmp(&0) {
            Ordering::Greater => 1,
            _ => 0,
        };
    let enemy_turns = player.hp / max(enemy.damage - player.armor, 1)
        + match (player.hp / max(enemy.damage - player.armor, 1)).cmp(&0) {
            Ordering::Greater => 1,
            _ => 0,
        };
    match me_turns.cmp(&enemy_turns) {
        Ordering::Greater => Winner::Enemy,
        _ => Winner::Player,
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let enemy = input_to_enemy(input)?;
    let mut equipment = build_equipment();
    equipment.sort_by(|a, b| a.cost().cmp(&b.cost()));

    for equip in equipment {
        match do_battle(
            &Combatant {
                hp: 100,
                damage: equip.damage(),
                armor: equip.armor(),
            },
            &enemy,
        ) {
            Winner::Player => return Ok(equip.cost()),
            Winner::Enemy => (),
        }
    }
    bail!("I can't win");
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let enemy = input_to_enemy(input)?;
    let mut equipment = build_equipment();
    equipment.sort_by(|a, b| b.cost().cmp(&a.cost()));

    for equip in equipment {
        match do_battle(
            &Combatant {
                hp: 100,
                damage: equip.damage(),
                armor: equip.armor(),
            },
            &enemy,
        ) {
            Winner::Player => (),
            Winner::Enemy => return Ok(equip.cost()),
        }
    }
    bail!("I can't lose");
}
