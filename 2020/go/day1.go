package advent2020

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func Day1(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 1: Report Repair")
	part1, err := day1Part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := day1Part2(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func inputToIntSlice(input string) ([]int, error) {
	lines := strings.Split(input, "\n")
	entries := make([]int, len(lines))
	var err error
	for i, line := range lines {
		entries[i], err = strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
	}
	return entries, nil
}

func day1Part1(input string) (int, error) {
	entries, err := inputToIntSlice(input)
	if err != nil {
		return 0, err
	}

	for i, x := range entries {
		for _, y := range entries[i+1:] {
			if x+y == 2020 {
				return x * y, nil
			}
		}
	}
	return 0, fmt.Errorf("Solution not found")
}

func day1Part2(input string) (int, error) {
	entries, err := inputToIntSlice(input)
	if err != nil {
		return 0, err
	}

	for i, x := range entries {
		for j, y := range entries[i+1:] {
			for _, z := range entries[j+1:] {
				if x+y+z == 2020 {
					return x * y * z, nil
				}
			}
		}
	}
	return 0, fmt.Errorf("Solution not found")
}
