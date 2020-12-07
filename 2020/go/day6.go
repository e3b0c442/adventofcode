package advent2020

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func Day6(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 6: Custom Customs")
	part1, err := day6Part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := day6Part2(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func day6Part1(input string) (int, error) {
	sum := 0
	for _, group := range strings.Split(input, "\n\n") {
		qs := make(map[rune]struct{})
		for _, line := range strings.Split(group, "\n") {
			for _, c := range line {
				qs[c] = struct{}{}
			}
		}
		for range qs {
			sum++
		}
	}
	return sum, nil
}

func day6Part2(input string) (int, error) {
	sum := 0
	for _, group := range strings.Split(input, "\n\n") {
		qs := make(map[rune]int)
		groups := strings.Split(group, "\n")
		for _, line := range strings.Split(group, "\n") {
			for _, c := range line {
				if _, ok := qs[c]; !ok {
					qs[c] = 0
				}
				qs[c]++
			}
		}
		for _, v := range qs {
			if v == len(groups) {
				sum++
			}
		}
	}
	return sum, nil
}
