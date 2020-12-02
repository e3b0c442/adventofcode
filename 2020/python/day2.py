#!/usr/bin/env python3
import re
import sys


def day2(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 2: Password Philosophy")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


pass_re = re.compile(r"(\d+)-(\d+) ([a-z]): (.*)")


def __part1(input):
    valid = 0
    for line in input.splitlines():
        m = pass_re.fullmatch(line)
        if m is None:
            raise Exception(f"Invalid input: {line}")

        (min, max, letter) = (int(m.group(1)), int(m.group(2)), m.group(3))
        count = 0
        for c in m.group(4):
            if c == letter:
                count += 1

        if min <= count <= max:
            valid += 1

    return valid


def __part2(input):
    valid = 0
    for line in input.splitlines():
        m = pass_re.fullmatch(line)
        if m is None:
            raise Exception(f"Invalid input: {line}")

        (l, r, c, pw) = (
            int(m.group(1)) - 1,
            int(m.group(2)) - 1,
            m.group(3),
            m.group(4),
        )
        if (pw[l] == c or pw[r] == c) and pw[l] != pw[r]:
            valid += 1

    return valid


if __name__ == "__main__":
    day2(sys.argv[1])