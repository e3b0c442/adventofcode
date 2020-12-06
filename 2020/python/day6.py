#!/usr/bin/env python3
import sys


def day6(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 6: Custom Customs")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    sum = 0

    groups = input.split("\n\n")
    for group in groups:
        map = {}
        for line in group.splitlines():
            for c in line:
                map[c] = True
        sum += len(map.keys())

    return sum


def __part2(input):
    sum = 0

    groups = input.split("\n\n")
    for group in groups:
        map = {}
        for line in group.splitlines():
            for c in line:
                try:
                    map[c] += 1
                except KeyError:
                    map[c] = 1
        all = 0
        groupsize = len(group.splitlines())
        for k, v in map.items():
            if v == groupsize:
                all += 1

        sum += all

    return sum


if __name__ == "__main__":
    day6(sys.argv[1])