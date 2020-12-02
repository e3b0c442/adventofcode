package advent2020

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

var passRE = regexp.MustCompile(`(\d+)-(\d+) ([a-z]): (.*)`)

func Day2(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 2: Password Philosophy")
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

func day2Part1(input string) (int, error) {
	valid := 0
	for _, line := range strings.Split(input, "\n") {
		m := passRE.FindStringSubmatch(line)

		min, err := strconv.Atoi(m[1])
		if err != nil {
			return 0, err
		}
		max, err := strconv.Atoi(m[2])
		if err != nil {
			return 0, err
		}
		ch := m[3][0]
		pw := m[4]

		count := 0
		for _, c := range []byte(pw) {
			if c == ch {
				count++
			}
		}
		if min <= count && count <= max {
			valid++
		}
	}

	return valid, nil
}

func day2Part2(input string) (int, error) {
	valid := 0
	for _, line := range strings.Split(input, "\n") {
		m := passRE.FindStringSubmatch(line)

		l, err := strconv.Atoi(m[1])
		if err != nil {
			return 0, err
		}
		l--
		r, err := strconv.Atoi(m[2])
		if err != nil {
			return 0, err
		}
		r--
		c := m[3][0]
		pw := []byte(m[4])

		if (pw[l] == c || pw[r] == c) && pw[l] != pw[r] {
			valid++
		}
	}

	return valid, nil
}
