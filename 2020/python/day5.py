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
        row = [0, 127]
        for rpart in line[:7]:
            split = (row[1] + 1 - row[0]) // 2
            if rpart == "F":
                row[1] -= split
            elif rpart == "B":
                row[0] += split
            else:
                raise Exception("Invalid input")

        col = [0, 7]
        for cpart in line[7:]:
            split = (col[1] + 1 - col[0]) // 2
            if cpart == "L":
                col[1] -= split
            elif cpart == "R":
                col[0] += split
            else:
                raise Exception("Invalid input")

        id = row[0] * 8 + col[0]
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