#!/usr/bin/env python3
import sys


def day5(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 5: Binary Boarding")
    ids = __part1(input)
    print(f"\tPart 1: {ids[-1]}")
    print(f"\tPart 2: {__part2(ids)}")


def __part1(input):
    ids = []
    for line in input.splitlines():
        rows = list(range(128))
        for rpart in line[:7]:
            split = len(rows) // 2
            if rpart == "F":
                rows = rows[:split]
            elif rpart == "B":
                rows = rows[split:]
            else:
                raise Exception("Invalid input")
        row = rows[0]

        cols = list(range(8))
        for cpart in line[7:]:
            split = len(cols) // 2
            if cpart == "L":
                cols = cols[:split]
            elif cpart == "R":
                cols = cols[split:]
            else:
                raise Exception("Invalid input")
        col = cols[0]

        id = row * 8 + col
        ids.append(id)

    ids.sort()
    return ids


def __part2(input):
    for i in range(len(input) - 2):
        if input[i] + 1 != input[i + 1]:
            return input[i] + 1

    raise Exception("Solution not found")


if __name__ == "__main__":
    day5(sys.argv[1])