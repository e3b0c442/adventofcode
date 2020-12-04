#!/usr/bin/env node

import fs from "fs";

const inputToPassports = (input) => {
  const records = input.split("\n\n");
  return records.map((record) => {
    return record
      .replace(/\n/g, " ")
      .split(" ")
      .reduce((passport, field) => {
        const kv = field.split(":");
        return { ...passport, [kv[0]]: kv[1] };
      }, {});
  });
};

const part1 = (input) => {
  return inputToPassports(input).reduce(
    (val, pp) =>
      (val +=
        "byr" in pp &&
        "iyr" in pp &&
        "eyr" in pp &&
        "hgt" in pp &&
        "hcl" in pp &&
        "ecl" in pp &&
        "pid" in pp
          ? 1
          : 0),
    0
  );
};

const validatePassport = (passport) => {
    try {
        if(passport.byr.length != 4) return false;
        const byr = parseInt(passport.byr);
        if(byr < 1920 || byr > 2002) return false;
        if(passport.iyr.length != 4) return false;
        const iyr = parseInt(passport.iyr);
        if(iyr < 2010 || iyr > 2020) return false;
        if(passport.eyr.length != 4) return false;
        const eyr = parseInt(passport.eyr);
        if(eyr < 2020 || eyr > 2030) return false;
        const [units, val] = [passport.hgt.slice(-2), parseInt(passport.hgt.slice(0, -2))];
        switch(units) {
            case "cm":
                if (val < 150 || val > 193) return false;
                break;
            case "in":
                if (val < 59 || val > 76) return false;
                break;
            default:
                return false;
        }
        if(!passport.hcl.match(/#[0-9a-f]{6}/)) return false;
        if(!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(passport.ecl)) return false;
        if(passport.pid.length != 9) return false;
        parseInt(passport.pid);
    } catch {
        return false;
    }
    return true;
}

const part2 = (input) => {
    return inputToPassports(input).filter(passport => validatePassport(passport)).length
};

const day4 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 4: Passport Processing");
  console.log(`\tPart 1: ${part1(input)}`);
  console.log(`\tPart 2: ${part2(input)}`);
};

export default day4;

try {
  day4(process.argv[2]);
} catch (e) {
  console.log(`${e}`);
}
