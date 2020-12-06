package advent2020

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

func Day5(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 5: Binary Boarding")
	part1, err := day5Part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1[len(part1)-1])

	part2, err := day5Part2(part1)
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func day5Part1(input string) ([]int, error) {
	lines := strings.Split(input, "\n")
	ids := make([]int, len(lines))

	for i, line := range lines {
		row := []int{0, 127}
		for _, c := range line[:7] {
			split := (row[1] + 1 - row[0]) / 2
			switch c {
			case 'F':
				row[1] -= split
			case 'B':
				row[0] += split
			default:
				return nil, fmt.Errorf("Invalid input %s", line)
			}
		}

		col := []int{0, 7}
		for _, c := range line[7:] {
			split := (col[1] + 1 - col[0]) / 2
			switch c {
			case 'L':
				col[1] -= split
			case 'R':
				col[0] += split
			default:
				return nil, fmt.Errorf("Invalid input %s", line)
			}
		}
		ids[i] = row[0]*8 + col[0]
	}
	sort.Ints(ids)
	return ids, nil
}

func day5Part2(ids []int) (int, error) {
	for i := range ids {
		if i != len(ids)-1 {
			if ids[i+1] != ids[i]+1 {
				return ids[i] + 1, nil
			}
		}
	}
	return 0, fmt.Errorf("Solution not found")
}
