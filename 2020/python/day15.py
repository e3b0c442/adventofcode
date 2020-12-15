#!/usr/bin/env python3
import sys
from functools import reduce


def day15(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 15: Rambunctious Recitation")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    map = {}
    starting = [int(x) for x in input.split(",")]
    next = 0
    for i, c in enumerate(starting):
        try:
            next = i - map[c]
        except KeyError:
            next = 0
        map[c] = i

    for i in range(len(starting), 2019):
        cur = next
        try:
            next = i - map[cur]
        except KeyError:
            next = 0
        map[cur] = i

    return next


def __part2(input):
    map = {}
    starting = [int(x) for x in input.split(",")]
    next = 0
    for i, c in enumerate(starting):
        try:
            next = i - map[c]
        except KeyError:
            next = 0
        map[c] = i

    for i in range(len(starting), 29999999):
        cur = next
        try:
            next = i - map[cur]
        except KeyError:
            next = 0
        map[cur] = i

    return next


if __name__ == "__main__":
    day15(sys.argv[1])