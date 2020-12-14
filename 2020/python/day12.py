#!/usr/bin/env python3
import sys


def day12(input_file):
    with open(input_file, "r") as f:
        input = f.read()

    print("Day 12: Rain Risk")
    print(f"\tPart 1: {__part1(input)}")
    print(f"\tPart 2: {__part2(input)}")


def __part1(input):
    dirs = ["E", "S", "W", "N"]
    heading = 0
    (x, y) = 0, 0

    for line in input.splitlines():
        (cmd, dist) = (line[0:1], int(line[1:]))
        dir = dirs[heading]

        if cmd == "E" or cmd == "S" or cmd == "W" or cmd == "N":
            dir = cmd
        elif cmd == "R":
            heading += dist // 90
            if heading > 3:
                heading -= 4
            dist = 0
        elif cmd == "L":
            heading -= dist // 90
            if heading < 0:
                heading += 4
            dist = 0
        elif cmd == "F":
            pass
        else:
            raise Exception("Invalid input")

        if dir == "E":
            x += dist
        elif dir == "S":
            y -= dist
        elif dir == "W":
            x -= dist
        elif dir == "N":
            y += dist

    return abs(x) + abs(y)


def __part2(input):
    (x, y, wx, wy) = 0, 0, 10, 1

    for line in input.splitlines():
        (cmd, dist) = (line[0:1], int(line[1:]))

        if cmd == "E":
            wx += dist
        elif cmd == "S":
            wy -= dist
        elif cmd == "W":
            wx -= dist
        elif cmd == "N":
            wy += dist
        elif cmd == "R":
            if dist == 90:
                (wx, wy) = (wy, -wx)
            elif dist == 180:
                (wx, wy) = (-wx, -wy)
            elif dist == 270:
                (wx, wy) = (-wy, wx)
            elif dist == 360:
                pass
            else:
                raise Exception("Invalid input")
        elif cmd == "L":
            if dist == 90:
                (wx, wy) = (-wy, wx)
            elif dist == 180:
                (wx, wy) = (-wx, -wy)
            elif dist == 270:
                (wx, wy) = (wy, -wx)
            elif dist == 360:
                pass
            else:
                raise Exception("Invalid input")
        elif cmd == "F":
            x += dist * wx
            y += dist * wy
        else:
            raise Exception("Invalid input")

    return abs(x) + abs(y)


if __name__ == "__main__":
    day12(sys.argv[1])