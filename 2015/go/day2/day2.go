package day2

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

//Day2 I Was Told There Would Be No Math
func Day2(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 2: I Was Told There Would Be No Math")
	part1, err := part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := part2(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func part1(input string) (int, error) {
	paper := 0
	for _, line := range strings.Split(input, "\n") {
		dimStrs := strings.Split(line, "x")
		if len(dimStrs) < 3 {
			return 0, fmt.Errorf("Invalid input: %s", line)
		}
		dims := make([]int, 3)
		var err error
		for i, dimStr := range dimStrs {
			dims[i], err = strconv.Atoi(dimStr)
			if err != nil {
				return 0, err
			}
		}
		sort.Ints(dims)
		paper += 2*dims[0]*dims[1] + 2*dims[1]*dims[2] + 2*dims[2]*dims[0] + dims[0]*dims[1]
	}
	return paper, nil
}

func part2(input string) (int, error) {
	ribbon := 0
	for _, line := range strings.Split(input, "\n") {
		dimStrs := strings.Split(line, "x")
		if len(dimStrs) < 3 {
			return 0, fmt.Errorf("Invalid input: %s", line)
		}
		dims := make([]int, 3)
		var err error
		for i, dimStr := range dimStrs {
			dims[i], err = strconv.Atoi(dimStr)
			if err != nil {
				return 0, err
			}
		}
		sort.Ints(dims)
		ribbon += 2*dims[0] + 2*dims[1] + dims[0]*dims[1]*dims[2]
	}
	return ribbon, nil
}
