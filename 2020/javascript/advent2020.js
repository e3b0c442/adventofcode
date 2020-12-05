#!/usr/bin/env node

import day1 from "./day1.js";
import day2 from "./day2.js";
import day3 from "./day3.js";
import day4 from "./day4.js";
import day5 from "./day5.js";

const days = [day1, day2, day3, day4, day5];

days.forEach((day, i) => {
  day(`${process.argv[2]}/${i + 1}.txt`);
});
