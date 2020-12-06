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

func rangeSlice(lo, hi int) []int {
	ints := make([]int, hi-lo)
	for i := lo; i < hi; i++ {
		ints[i] = i
	}
	return ints
}

func day5Part1(input string) ([]int, error) {
	lines := strings.Split(input, "\n")
	ids := make([]int, len(lines))

	for i, line := range lines {
		rows := rangeSlice(0, 128)
		for _, c := range line[:7] {
			split := len(rows) / 2
			switch c {
			case 'F':
				rows = rows[:split]
			case 'B':
				rows = rows[split:]
			default:
				return nil, fmt.Errorf("Invalid input %s", line)
			}
		}
		row := rows[0]

		cols := rangeSlice(0, 8)
		for _, c := range line[7:] {
			split := len(cols) / 2
			switch c {
			case 'L':
				cols = cols[:split]
			case 'R':
				cols = cols[split:]
			default:
				return nil, fmt.Errorf("Invalid input %s", line)
			}
		}
		col := cols[0]
		ids[i] = row*8 + col
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
