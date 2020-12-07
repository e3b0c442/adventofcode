#!/usr/bin/env node

const fs = require("fs");

const part1 = (input) => {
  return input
    .split("\n")
    .map((p) => {
      let row = [0, 127];
      for (let c of p.slice(0, 7)) {
        const split = (row[1] + 1 - row[0]) / 2;
        switch (c) {
          case "F":
            row[1] -= split;
            break;
          case "B":
            row[0] += split;
            break;
          default:
            throw "Invalid input";
        }
      }

      let col = [0, 7];
      for (let c of p.slice(7)) {
        const split = (col[1] + 1 - col[0]) / 2;
        switch (c) {
          case "L":
            col[1] -= split;
            break;
          case "R":
            col[0] += split;
            break;
          default:
            throw "Invalid input";
        }
      }

      return row[0] * 8 + col[0];
    })
    .sort((a, b) => a - b);
};

const part2 = (ids) => {
  for (let i = 0; i < ids.length - 1; i++) {
    if (ids[i] + 1 != ids[i + 1]) {
      return ids[i] + 1;
    }
  }
  throw "Solution not found";
};

const day5 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 5: Binary Boarding");
  const ids = part1(input);
  console.log(`\tPart 1: ${ids[ids.length - 1]}`);
  console.log(`\tPart 2: ${part2(ids)}`);
};

module.exports = day5;

if (require.main === module) day5(process.argv[2]);
