#!/usr/bin/env python3
import sys
from functools import reduce


def day13(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 13: Shuttle Search")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    lines = input.splitlines()
    target = int(lines[0])
    earliest = 0
    wait = sys.maxsize

    buses = reduce(
        lambda ins, b: ins + [int(b)] if b != "x" else ins, lines[1].split(","), []
    )
    for bus in buses:
        w = bus - (target % bus)
        if w < wait:
            wait = w
            earliest = bus

    return earliest * wait


def __part2(input):
    schedule = input.splitlines()[1].split(",")
    buses = reduce(
        lambda m, e: m + [(int(e[1]), e[0])] if e[1].isdigit() else m,
        enumerate(schedule),
        [],
    )

    t = 100000000000000
    increment = 1
    while len(buses) > 0:
        (n, buses) = (buses[0], buses[1:])
        while True:
            diff = n[0] - (t % n[0])
            if diff == n[0]:
                diff = 0

            if diff == n[1] % n[0]:
                increment *= n[0]
                break

            t += increment

    return t


if __name__ == "__main__":
    day13(sys.argv[1])