#!/usr/bin/env python3
import sys
from day1 import day1
from day2 import day2
from day3 import day3
from day4 import day4
from day5 import day5
from day6 import day6
from day7 import day7
from day8 import day8


def advent2020(input_folder):
    funcs = [day1, day2, day3, day4, day5, day6, day7, day8]
    for i, f in enumerate(funcs):
        funcs[i](f"{input_folder}/{i+1}.txt")


if __name__ == "__main__":
    advent2020(sys.argv[1])