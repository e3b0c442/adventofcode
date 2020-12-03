package advent2019

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

func recurseFuel(val int) int {
	addl := val/3 - 2
	if addl <= 0 {
		return 0
	}

	return addl + recurseFuel(addl)
}

func day1Part1(input string) (int, error) {
	mods, err := inputToIntSlice(input)
	if err != nil {
		return 0, err
	}

	fuel := 0
	for _, mod := range mods {
		fuel += mod/3 - 2
	}
	return fuel, nil
}

func day1Part2(input string) (int, error) {
	mods, err := inputToIntSlice(input)
	if err != nil {
		return 0, err
	}

	fuel := 0
	for _, mod := range mods {
		fuel += recurseFuel(mod)
	}
	return fuel, nil
}
