#!/usr/bin/env node

import fs from "fs";

const part1 = (input) => {
  return input
    .split("\n")
    .map((p) => {
      let rows = [...Array(128).keys()];
      for (let c of p.slice(0, 7)) {
        const split = rows.length / 2;
        switch (c) {
          case "F":
            rows = rows.slice(0, split);
            break;
          case "B":
            rows = rows.slice(split);
            break;
          default:
            throw "Invalid input";
        }
      }
      const row = parseInt(rows[0]);

      let cols = [...Array(8).keys()];
      for (let c of p.slice(7)) {
        const split = cols.length / 2;
        switch (c) {
          case "L":
            cols = cols.slice(0, split);
            break;
          case "R":
            cols = cols.slice(split);
            break;
          default:
            throw "Invalid input";
        }
      }
      const col = parseInt(cols[0]);
      return row * 8 + col;
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

export default day5;

try {
  day5(process.argv[2]);
} catch (e) {
  console.log(`${e}`);
}
