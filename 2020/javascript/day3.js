#!/usr/bin/env node

const fs = require("fs");

const slope_trees = (rows, slope) => {
  const [r, d] = slope;
  let x = 0;
  let trees = 0;
  for (let i = 0; i < rows.length; i += d) {
    if (x >= rows[i].length) {
      x = x - rows[i].length;
    }
    if (rows[i][x] === "#") {
      trees++;
    }
    x += r;
  }
  return trees;
};
const part1 = (input) => slope_trees(input.split("\n"), [3, 1]);

const part2 = (input) =>
  [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2],
  ].reduce((acc, cur) => acc * slope_trees(input.split("\n"), cur), 1);

const day3 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 3: Toboggan Trajectory");
  console.log(`\tPart 1: ${part1(input)}`);
  console.log(`\tPart 2: ${part2(input)}`);
};

module.exports = day3;

if (require.main === module) day3(process.argv[2]);
