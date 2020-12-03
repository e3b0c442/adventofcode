#!/usr/bin/env python3
import sys
import functools

def day3(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 3: Toboggan Trajectory")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")

def __slope_trees(rows, slope):
        (r, d) = slope
        x = 0
        trees = 0
        for i in range(0, len(rows), d):
            if x >= len(rows[i]):
                x = x - len(rows[i])
            if rows[i][x] == "#":
                trees += 1
            x += r
        return trees

def __part1(input):
    return __slope_trees(input.splitlines(), (3,1))

def __part2(input):
    slopes = (
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    )

    return functools.reduce(lambda acc, cur: acc * __slope_trees(input.splitlines(), cur), slopes, 1)


if __name__ == "__main__":
    day3(sys.argv[1])