#!/usr/bin/env node

const fs = require("fs");

const part1 = (input) => {
  let valid = 0;
  input.split("\n").forEach((line) => {
    const caps = line.match(/(\d+)-(\d+) ([a-z]): (.*)/);
    const [min, max, ch, pw] = [
      parseInt(caps[1]),
      parseInt(caps[2]),
      caps[3][0],
      caps[4],
    ];

    let count = 0;
    for (let c of pw) {
      count += c === ch ? 1 : 0;
    }
    if (min <= count && count <= max) {
      valid++;
    }
  });

  return valid;
};

const part2 = (input) => {
  let valid = 0;
  input.split("\n").forEach((line) => {
    const caps = line.match(/(\d+)-(\d+) ([a-z]): (.*)/);
    const [l, r, c, pw] = [
      parseInt(caps[1]) - 1,
      parseInt(caps[2]) - 1,
      caps[3][0],
      caps[4],
    ];

    if ((pw[l] === c || pw[r] === c) && pw[l] !== pw[r]) {
      valid++;
    }
  });

  return valid;
};

const day2 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 2: Password Philosophy");
  console.log(`\tPart 1: ${part1(input)}`);
  console.log(`\tPart 2: ${part2(input)}`);
};

module.exports = day2;

if (require.main === module) day2(process.argv[2]);
