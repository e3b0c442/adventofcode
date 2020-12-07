#!/usr/bin/env node

const day1 = require("./day1.js");
const day2 = require("./day2.js");
const day3 = require("./day3.js");
const day4 = require("./day4.js");
const day5 = require("./day5.js");
const day6 = require("./day6.js");

const days = [day1, day2, day3, day4, day5, day6];

days.forEach((day, i) => {
  day(`${process.argv[2]}/${i + 1}.txt`);
});
