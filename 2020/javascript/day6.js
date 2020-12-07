#!/usr/bin/env node

const fs = require("fs");

const part1 = (input) => {
  return input.split("\n\n").reduce((sum, group) => {
    return (
      sum +
      Object.keys(
        group.split("\n").reduce((set, line) => {
          return {
            ...[...line].reduce((qs, c) => {
              return { ...qs, [c]: true };
            }, set),
          };
        }, {})
      ).length
    );
  }, 0);
};

const part2 = (input) => {
  return input.split("\n\n").reduce((sum, group) => {
    return (
      sum +
      Object.entries(
        group.split("\n").reduce((set, line) => {
          return {
            ...[...line].reduce((qs, c) => {
              return { ...qs, [c]: qs[c] ? qs[c] + 1 : 1 };
            }, set),
          };
        }, {})
      ).filter(([k, v]) => v == group.split("\n").length).length
    );
  }, 0);
};

const day6 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 6: Custom Customs");
  console.log(`\tPart 1: ${part1(input)}`);
  console.log(`\tPart 2: ${part2(input)}`);
};

module.exports = day6;

if (require.main === module) day6(process.argv[2]);
