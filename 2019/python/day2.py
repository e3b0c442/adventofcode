#!/usr/bin/env python3
import sys


def day2(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 2: 1202 Program Alarm")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __run_intcode(intcode):
    cursor = 0
    op = intcode[cursor]
    while op != 99:
        if op == 1:
            intcode[intcode[cursor + 3]] = (
                intcode[intcode[cursor + 1]] + intcode[intcode[cursor + 2]]
            )
        elif op == 2:
            intcode[intcode[cursor + 3]] = (
                intcode[intcode[cursor + 1]] * intcode[intcode[cursor + 2]]
            )
        else:
            raise Exception(f"Invalid opcode {op}")
        cursor += 4
        op = intcode[cursor]
    return intcode[0]


def __part1(input):
    intcode = [int(x) for x in input.split(",")]
    intcode[1] = 12
    intcode[2] = 2

    return __run_intcode(intcode)


def __part2(input):
    for i in range(100):
        for j in range(100):
            intcode = [int(x) for x in input.split(",")]
            intcode[1] = i
            intcode[2] = j

            if __run_intcode(intcode) == 19690720:
                return 100 * i + j


if __name__ == "__main__":
    day2(sys.argv[1])