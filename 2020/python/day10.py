#!/usr/bin/env python3
import sys
from functools import reduce


def day10(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 10: Adapter Array")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    input = [0] + sorted([int(x) for x in input.splitlines()])
    input.append(input[-1] + 3)
    diffs = [0 for x in range(3)]
    for (i, _) in enumerate(input):
        if i == len(input) - 1:
            break
        diffs[input[i + 1] - input[i] - 1] += 1

    return diffs[1 - 1] * diffs[3 - 1]


def __build_arrangements(n, unused, cache):
    if len(unused) == 0:
        return 1

    try:
        return cache[n]
    except KeyError:
        pass

    arrangements = 0
    for (i, c) in enumerate(unused):
        if c - n > 3:
            break
        arrangements += __build_arrangements(c, unused[i + 1 :], cache)

    cache[n] = arrangements
    return arrangements


def __part2(input):
    input = sorted([int(x) for x in input.splitlines()])
    possibilities = __build_arrangements(0, input, {})
    return possibilities


if __name__ == "__main__":
    day10(sys.argv[1])