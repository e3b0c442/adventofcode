#!/usr/bin/env node

import fs from "fs";

const part1 = (input) => {
  const mods = input.split("\n").map((e) => parseInt(e));
  return mods.reduce((acc, cur) => acc + (Math.trunc(cur / 3) - 2), 0);
};

const recurseFuel = (val) =>
  Math.trunc(val / 3) - 2 <= 0
    ? 0
    : Math.trunc(val / 3) - 2 + recurseFuel(Math.trunc(val / 3) - 2);

const part2 = (input) => {
  const mods = input.split("\n").map((e) => parseInt(e));
  return mods.reduce((acc, cur) => acc + recurseFuel(cur), 0);
};

const day1 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 1: The Tyranny of the Rocket Equation");
  console.log(`\tPart 1: ${part1(input)}`);
  console.log(`\tPart 2: ${part2(input)}`);
};

export default day1;

try {
  day1(process.argv[2]);
} catch (e) {
  console.log(`${e}`);
}
