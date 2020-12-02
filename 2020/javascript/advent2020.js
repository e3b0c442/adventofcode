#!/usr/bin/env node

import day1 from "./day1.js";
import day2 from "./day2.js";

const days = [day1, day2];

days.forEach((day, i) => {
  day(`${process.argv[2]}/${i + 1}.txt`);
});
