#!/usr/bin/env python3
import sys
from day1 import day1
from day2 import day2


def advent2020(input_folder):
    funcs = [day1, day2]
    for i, f in enumerate(funcs):
        funcs[i](f"{input_folder}/{i+1}.txt")


if __name__ == "__main__":
    advent2020(sys.argv[1])