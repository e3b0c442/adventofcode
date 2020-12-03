#!/usr/bin/env python3
import sys


def day1(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 1: The Tyranny of the Rocket Equation")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")

def __recurse_fuel(val):
    return 0 if val//3-2 <= 0 else val//3-2 + __recurse_fuel(val//3-2)


def __part1(input):
    input = [int(x) for x in input.splitlines()]

    fuel = 0
    for mod in input:
        fuel += mod//3-2

    return fuel


def __part2(input):
    input = [int(x) for x in input.splitlines()]

    fuel = 0
    for mod in input:
        fuel += __recurse_fuel(mod)
    
    return fuel


if __name__ == "__main__":
    day1(sys.argv[1])