#!/usr/bin/env python3
import sys
from functools import reduce


def day9(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 9: Encoding Error")
    part1 = __part1(input)
    print(f"\tPart 1: {part1}")
    print(f"\tPart 2: {__part2(input, part1)}")


def __part1(input):
    seed = []
    sums = []
    for line in input.splitlines():
        v = int(line)
        if len(seed) == 25:
            if v not in sums:
                return v

        seed.append(v)
        if len(seed) < 25:
            continue
        if len(seed) > 25:
            seed = seed[-25:]
        sums = []
        for i in range(0, len(seed)):
            for j in range(i, len(seed)):
                sums.append(seed[i] + seed[j])


def __part2(input, notsum):
    nums = list(reversed([int(x) for x in input.splitlines()]))
    for l in range(len(nums)):
        for r in range(l + 1, len(nums)):
            sl = nums[l : r + 1]
            sum = reduce(lambda sum, x: sum + x, sl, 0)
            if sum == notsum:
                sl.sort()
                return sl[0] + sl[-1]
            if sum > notsum:
                break


if __name__ == "__main__":
    day9(sys.argv[1])