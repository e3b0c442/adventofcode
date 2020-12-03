package advent2020

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func Day3(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 3: Toboggan Trajectory")
	part1, err := day3Part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := day3Part2(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func slopeTrees(rows []string, slope []int) int {
	x, trees := 0, 0
	r, d := slope[0], slope[1]
	for i := 0; i < len(rows); i += d {
		if x >= len(rows[i]) {
			x = x - len(rows[i])
		}
		if rows[i][x] == '#' {
			trees++
		}
		x += r
	}
	return trees
}

func day3Part1(input string) (int, error) {
	return slopeTrees(strings.Split(input, "\n"), []int{3, 1}), nil
}

func day3Part2(input string) (int, error) {
	acc := 1
	for _, s := range [][]int{{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}} {
		acc *= slopeTrees(strings.Split(input, "\n"), s)
	}
	return acc, nil
}
