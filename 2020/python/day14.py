#!/usr/bin/env python3
import sys
from functools import reduce


def day14(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 14: Docking Data")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __parse_mask(mask):
    mask = mask[::-1]
    zeros = 0b00001111_11111111_11111111_11111111_11111111
    ones = 0b00000000_00000000_00000000_00000000_00000000
    for (i, c) in enumerate(mask):
        if c == "0":
            zeros &= ~(1 << i)
        elif c == "1":
            ones |= 1 << i
    return (zeros, ones)


def __part1(input):
    mask = (0, 0)
    vals = {}
    for line in input.splitlines():
        if line.startswith("mask"):
            line.lstrip("mask = ")
            mask = __parse_mask(line)
        elif line.startswith("mem["):
            (addr, val) = line.lstrip("mem[").split("] = ")
            (addr, val) = (int(addr), int(val))
            vals[addr] = (val & mask[0]) | mask[1]
        else:
            raise Exception(f"Invalid input: {line}")

    return sum(vals.values())


def __parse_masks(mask):
    masks = [mask]
    for (i, c) in enumerate(mask):
        if c == "0":
            news = []
            for m in masks:
                news += [m[:i] + "X" + m[i + 1 :]]
            masks = news
        if c == "X":
            news = []
            for m in masks:
                news += [m[:i] + "0" + m[i + 1 :]]
                news += [m[:i] + "1" + m[i + 1 :]]
            masks = news
    return [__parse_mask(x) for x in masks]


def __part2(input):
    masks = []
    vals = {}
    for line in input.splitlines():
        if line.startswith("mask"):
            line.lstrip("mask = ")
            masks = __parse_masks(line)
        elif line.startswith("mem["):
            (baseaddr, val) = line.lstrip("mem[").split("] = ")
            (baseaddr, val) = (int(baseaddr), int(val))
            addrs = [(baseaddr & x[0]) | x[1] for x in masks]
            for addr in addrs:
                vals[addr] = val
        else:
            raise Exception(f"Invalid input: {line}")

    return sum(vals.values())


if __name__ == "__main__":
    day14(sys.argv[1])