#!/usr/bin/env python3
import sys
from day1 import day1


def advent2019(input_folder):
    funcs = [day1]
    for i, f in enumerate(funcs):
        funcs[i](f"{input_folder}/{i+1}.txt")


if __name__ == "__main__":
    advent2019(sys.argv[1])