package day1

import (
	"errors"
	"fmt"
	"io/ioutil"
)

//Day1 Not Quite Lisp
func Day1(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 1: Not Quite Lisp")
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
	floor := 0
	for _, c := range input {
		switch c {
		case '(':
			floor++
		case ')':
			floor--
		}
	}
	return floor, nil
}

func part2(input string) (int, error) {
	floor := 0
	for i, c := range input {
		switch c {
		case '(':
			floor++
		case ')':
			floor--
		}
		if floor == -1 {
			return i + 1, nil
		}
	}
	return 0, errors.New("No solution")
}
