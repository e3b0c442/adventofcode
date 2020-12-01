#!/usr/bin/env python3
import sys


def day1(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 1: Report Repair")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    input = [int(x) for x in input.splitlines()]

    for i in range(len(input)):
        for j in range(i + 1, len(input)):
            if input[i] + input[j] == 2020:
                return input[i] * input[j]


def __part2(input):
    input = [int(x) for x in input.splitlines()]

    for i in range(len(input)):
        for j in range(i + 1, len(input)):
            for k in range(j + 1, len(input)):
                if input[i] + input[j] + input[k] == 2020:
                    return input[i] * input[j] * input[k]


if __name__ == "__main__":
    day1(sys.argv[1])