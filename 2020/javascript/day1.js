#!/usr/bin/env node

import fs from "fs";

const part1 = (input) => {
  const entries = input.split("\n").map((e) => parseInt(e));
  for (let i = 0; i < entries.length; i++) {
    for (let j = i + 1; j < entries.length; j++) {
      if (entries[i] + entries[j] === 2020) {
        return entries[i] * entries[j];
      }
    }
  }
};

const part2 = (input) => {
  const entries = input.split("\n").map((e) => parseInt(e));
  for (let i = 0; i < entries.length; i++) {
    for (let j = i + 1; j < entries.length; j++) {
      for (let k = j + 1; k < entries.length; k++) {
        if (entries[i] + entries[j] + entries[k] === 2020) {
          return entries[i] * entries[j] * entries[k];
        }
      }
    }
  }
};

const day1 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 1: Report Repair");
  console.log(`\tPart 1: ${part1(input)}`);
  console.log(`\tPart 2: ${part2(input)}`);
};

export default day1;

try {
  day1(process.argv[2]);
} catch (e) {
  console.log(`${e}`);
}
