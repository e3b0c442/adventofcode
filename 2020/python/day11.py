#!/usr/bin/env python3
import sys
from functools import reduce


def day11(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 11: Seating System")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    state = [[y for y in x] for x in list(input.splitlines())]
    (w, h) = (len(state), len(state[0]))

    while True:
        new = [[y for y in x] for x in state]

        changed = False
        for y in range(h):
            for x in range(w):
                seat = state[x][y]
                adj_occ = 0
                for xx in range(x - 1, x + 2):
                    for yy in range(y - 1, y + 2):
                        if (
                            xx < 0
                            or yy < 0
                            or xx >= w
                            or yy >= h
                            or (xx == x and yy == y)
                        ):
                            continue
                        if state[xx][yy] == "#":
                            adj_occ += 1

                if seat == "L":
                    if adj_occ == 0:
                        new[x][y] = "#"
                        changed = True
                elif seat == "#":
                    if adj_occ >= 4:
                        new[x][y] = "L"
                        changed = True
        state = new
        if changed is False:
            break

    return reduce(
        lambda s, x: s + reduce(lambda ss, y: ss + 1 if y == "#" else ss, x, 0),
        state,
        0,
    )


def __part2(input):
    state = [[y for y in x] for x in list(input.splitlines())]
    (w, h) = (len(state), len(state[0]))

    while True:
        new = [[y for y in x] for x in state]

        changed = False
        for y in range(h):
            for x in range(w):
                seat = state[x][y]
                occ = 0
                xx = x + 1
                while xx < w:
                    if state[xx][y] == "#":
                        occ += 1
                        break
                    if state[xx][y] == "L":
                        break
                    xx += 1
                xx = x + 1
                yy = y + 1
                while xx < w and yy < h:
                    if state[xx][yy] == "#":
                        occ += 1
                        break
                    if state[xx][yy] == "L":
                        break
                    xx += 1
                    yy += 1
                yy = y + 1
                while yy < h:
                    if state[x][yy] == "#":
                        occ += 1
                        break
                    if state[x][yy] == "L":
                        break
                    yy += 1
                xx = x - 1
                yy = y + 1
                while xx >= 0 and yy < h:
                    if state[xx][yy] == "#":
                        occ += 1
                        break
                    if state[xx][yy] == "L":
                        break
                    xx -= 1
                    yy += 1
                xx = x - 1
                while xx >= 0:
                    if state[xx][y] == "#":
                        occ += 1
                        break
                    if state[xx][y] == "L":
                        break
                    xx -= 1
                xx = x - 1
                yy = y - 1
                while xx >= 0 and yy >= 0:
                    if state[xx][yy] == "#":
                        occ += 1
                        break
                    if state[xx][yy] == "L":
                        break
                    xx -= 1
                    yy -= 1
                yy = y - 1
                while yy >= 0:
                    if state[x][yy] == "#":
                        occ += 1
                        break
                    if state[x][yy] == "L":
                        break
                    yy -= 1
                xx = x + 1
                yy = y - 1
                while yy >= 0 and xx < w:
                    if state[xx][yy] == "#":
                        occ += 1
                        break
                    if state[xx][yy] == "L":
                        break
                    xx += 1
                    yy -= 1
                if seat == "L":
                    if occ == 0:
                        new[x][y] = "#"
                        changed = True
                elif seat == "#":
                    if occ >= 5:
                        new[x][y] = "L"
                        changed = True
        state = new
        if changed is False:
            break

    return reduce(
        lambda s, x: s + reduce(lambda ss, y: ss + 1 if y == "#" else ss, x, 0),
        state,
        0,
    )


if __name__ == "__main__":
    day11(sys.argv[1])