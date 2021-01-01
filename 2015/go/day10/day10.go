package day10

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

//Day10 Elves Look, Elves Say
func Day10(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 10: Elves Look, Elves Say")
	part1, err := part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", len(part1))

	part2, err := part2(string(part1))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func part1(input string) (string, error) {
	b := strings.Builder{}
	for i := 0; i < 40; i++ {
		last := '\u0000'
		count := 0

		for _, c := range input {
			if c == last {
				count++
			} else {
				if count > 0 {
					b.WriteString(strconv.Itoa(count))
					b.WriteRune(last)
				}
				last = c
				count = 1
			}
		}
		b.WriteString(strconv.Itoa(count))
		b.WriteRune(last)

		input = b.String()
		b.Reset()
	}

	return input, nil
}

func part2(input string) (int, error) {
	b := strings.Builder{}
	for i := 40; i < 50; i++ {
		last := '\u0000'
		count := 0

		for _, c := range input {
			if c == last {
				count++
			} else {
				if count > 0 {
					b.WriteString(strconv.Itoa(count))
					b.WriteRune(last)
				}
				last = c
				count = 1
			}
		}
		b.WriteString(strconv.Itoa(count))
		b.WriteRune(last)

		input = b.String()
		b.Reset()
	}

	return len(input), nil
}
