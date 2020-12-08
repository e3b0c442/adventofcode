#!/usr/bin/env python3
import sys
import re
import pprint


def day7(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 7: Handy Haversacks)")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


container_re = re.compile(r"^([a-z]+ [a-z]+) bags contain (.*)\.$")
contained_re = re.compile(r"(\d)+ ([a-z]+ [a-z]+) bags?")


def find_golds(bag_map, trail):
    if "shiny gold" in bag_map[trail[-1]]:
        return True

    for x in bag_map[trail[-1]]:
        if x in trail:
            continue
        if find_golds(bag_map, trail + [x]):
            return True

    return False


def __part1(input):
    bag_map = {}
    for line in input.splitlines():
        bag_match = container_re.match(line)
        if bag_match is None:
            raise Exception("Invalid input")
        bagged_matches = contained_re.findall(bag_match.group(2))
        bag_map[bag_match.group(1)] = [x for (_, x) in bagged_matches]

    return len(
        list(
            filter(
                lambda x: x is True, [find_golds(bag_map, [x]) for x in bag_map.keys()]
            )
        )
    )


def count_in_gold(bag_map, key):
    count = 0
    for (k, v) in bag_map[key].items():
        count += v + v * count_in_gold(bag_map, k)

    return count


def __part2(input):
    bag_map = {}
    for line in input.splitlines():
        bag_match = container_re.match(line)
        if bag_match is None:
            raise Exception("Invalid input")
        bagged_matches = contained_re.findall(bag_match.group(2))
        bag_map[bag_match.group(1)] = {k: int(v) for (v, k) in bagged_matches}

    return count_in_gold(bag_map, "shiny gold")


if __name__ == "__main__":
    day7(sys.argv[1])