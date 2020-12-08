#!/usr/bin/env python3
import sys


def day8(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 8: Handheld Halting")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    instrs = [(y[0], int(y[1])) for y in [x.split(" ") for x in input.splitlines()]]
    acc = 0
    ip = 0
    visited = set()
    while True:
        if ip in visited:
            return acc
        visited.add(ip)
        (op, v) = instrs[ip]
        if op == "acc":
            acc += v
            ip += 1
        elif op == "jmp":
            ip += v
        elif op == "nop":
            ip += 1
        else:
            raise Exception("Invalid input")


def __part2(input):
    orig = [(y[0], int(y[1])) for y in [x.split(" ") for x in input.splitlines()]]
    for (i, _) in enumerate(orig):
        if orig[i][0] == "acc":
            continue

        instrs = [x for x in orig]
        if instrs[i][0] == "jmp":
            instrs[i] = ("nop", instrs[i][1])
        elif instrs[i][0] == "nop":
            instrs[i] = ("jmp", instrs[i][1])
        else:
            raise Exception("Invalid input")

        acc = 0
        ip = 0
        visited = set()
        found = False
        while True:
            if ip == len(instrs):
                found = True
                break
            if ip in visited:
                break
            visited.add(ip)
            (op, v) = instrs[ip]
            if op == "acc":
                acc += v
                ip += 1
            elif op == "jmp":
                ip += v
            elif op == "nop":
                ip += 1
            else:
                raise Exception("Invalid input")
        if found is True:
            return acc


if __name__ == "__main__":
    day8(sys.argv[1])