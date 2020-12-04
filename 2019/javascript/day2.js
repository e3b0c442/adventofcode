#!/usr/bin/env node

import fs from "fs";

const runIntcode = (intcode) => {
    let cursor = 0;
    while(true) {
        switch(intcode[cursor]) {
            case 1:
                intcode[intcode[cursor+3]] = intcode[intcode[cursor+1]] + intcode[intcode[cursor+2]];
                break;
            case 2:
                intcode[intcode[cursor+3]] = intcode[intcode[cursor+1]] * intcode[intcode[cursor+2]];
                break;
            case 99:
                return intcode[0];
                break;
            default:
                throw `Invalid opcode: ${intcode[cursor]}`
        }
        cursor += 4;
    }
}

const part1 = (input) => {
    let intcode = input.split(",").map(x => parseInt(x));
    intcode[1] = 12
    intcode[2] = 2
    return runIntcode(intcode);
}

const part2 = (input) => {
    let intcode = input.split(",").map(x => parseInt(x));
    for(let i = 0; i < 100; i++) {
        for (let j = 0; j < 100; j++) {
            let ic = [...intcode];
            ic[1] = i;
            ic[2] = j;
            if(runIntcode(ic) == 19690720) {
                return 100 * i + j;
            }
        }
    }
}

const day2 = (inputFile) => {
  const input = fs.readFileSync(inputFile, "utf8");

  console.log("Day 2: 1202 Program Alarm");
  console.log(`\tPart 1: ${part1(input)}`);
  console.log(`\tPart 2: ${part2(input)}`);
};

export default day2;

try {
  day2(process.argv[2]);
} catch (e) {
  console.log(`${e}`);
}
