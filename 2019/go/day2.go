package advent2019

import (
	"errors"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func Day2(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 2: 1202 Program Alarm")
	part1, err := day2Part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := day2Part2(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func inputToIntcode(input string) ([]int, error) {
	ints := strings.Split(input, ",")
	intcode := make([]int, len(ints))
	var err error
	for i, n := range ints {
		intcode[i], err = strconv.Atoi(n)
		if err != nil {
			return nil, err
		}
	}
	return intcode, nil
}

func runIntcode(intcode []int) (int, error) {
	//copy to avoid mangle
	ic := make([]int, len(intcode))
	copy(ic, intcode)
	ip := 0
	for {
		switch ic[ip] {
		case 1:
			ic[ic[ip+3]] = ic[ic[ip+1]] + ic[ic[ip+2]]
		case 2:
			ic[ic[ip+3]] = ic[ic[ip+1]] * ic[ic[ip+2]]
		case 99:
			return ic[0], nil
		default:
			return 0, fmt.Errorf("Invalid opcode %d", ic[ip])
		}
		ip += 4
	}
}

func day2Part1(input string) (int, error) {
	intcode, err := inputToIntcode(input)
	if err != nil {
		return 0, err
	}
	intcode[1] = 12
	intcode[2] = 2
	return runIntcode(intcode)
}

func day2Part2(input string) (int, error) {
	intcode, err := inputToIntcode(input)
	if err != nil {
		return 0, err
	}
	for i := 0; i < 100; i++ {
		for j := 0; j < 100; j++ {
			intcode[1] = i
			intcode[2] = j
			rval, err := runIntcode(intcode)
			if err != nil {
				return 0, err
			}
			if rval == 19690720 {
				return 100*i + j, nil
			}
		}
	}
	return 0, errors.New("No solution found")
}
