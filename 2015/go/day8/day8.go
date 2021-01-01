package day8

import (
	"fmt"
	"io/ioutil"
	"strings"
)

//Day8 Matchsticks
func Day8(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 8: Matchsticks")
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
	code := 0
	memory := 0
	in := false

	for i := 0; i < len(input); i++ {
		if !in {
			if input[i] == '"' {
				code++
				in = true
			}
		} else {
			code++
			switch input[i] {
			case '"':
				in = false
			case '\\':
				memory++
				i++
				switch input[i] {
				case '\\', '"':
					code++
				case 'x':
					code += 3
					i += 2
				}
			default:
				memory++
			}
		}
	}

	return code - memory, nil
}

func part2(input string) (int, error) {
	encoded := 0
	raw := 0
	for _, line := range strings.Split(input, "\n") {
		encoded += 2
		for _, c := range line {
			raw++
			switch c {
			case '"', '\\':
				encoded += 2
			default:
				encoded++
			}
		}
	}
	return encoded - raw, nil
}
