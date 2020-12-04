#!/usr/bin/env python3
import re
import sys


def day4(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 4: Passport Processing")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __input_to_records(input):
    records = []
    record = {}

    for line in input.splitlines():
        if line == "":
            records.append(record)
            record = {}
            continue
        fields = line.split(" ")
        for field in fields:
            kv = field.split(":")
            record[kv[0]] = kv[1]

    records.append(record)
    return records


hgt_re = re.compile(r"(\d+)(cm|in)")
hcl_re = re.compile(r"#[A-Fa-f0-9]{6}")


def __part1(input):
    valid = 0
    for record in __input_to_records(input):
        try:
            record["byr"]
            record["iyr"]
            record["eyr"]
            record["hgt"]
            record["hcl"]
            record["ecl"]
            record["pid"]
            valid += 1
        except KeyError:
            pass

    return valid


def __part2(input):
    valid = 0
    for record in __input_to_records(input):
        try:
            byr = int(record["byr"])
            if byr < 1920 or byr > 2002:
                raise ValueError
            iyr = int(record["iyr"])
            if iyr < 2010 or iyr > 2020:
                raise ValueError
            eyr = int(record["eyr"])
            if eyr < 2020 or eyr > 2030:
                raise ValueError
            hgt = hgt_re.fullmatch(record["hgt"])
            if hgt is None:
                raise ValueError
            else:
                u, v = hgt.group(2), int(hgt.group(1))
            if u == "cm":
                if not 150 <= v <= 193:
                    raise ValueError
            elif u == "in":
                if not 59 <= v <= 76:
                    raise ValueError
            else:
                raise ValueError
            if hcl_re.fullmatch(record["hcl"]) is None:
                raise ValueError
            if record["ecl"] not in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
                raise ValueError
            if len(record["pid"]) != 9:
                raise ValueError
            int(record["pid"])

            valid += 1
        except (KeyError, ValueError):
            pass

    return valid


if __name__ == "__main__":
    day4(sys.argv[1])