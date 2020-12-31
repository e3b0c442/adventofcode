package day5

import (
	"fmt"
	"io/ioutil"
	"strings"
)

//Day5 Doesn't He Have Intern-Elves For This?
func Day5(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 5: Doesn't He Have Intern-Elves For This?")
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
	nice := 0
outer:
	for _, line := range strings.Split(input, "\n") {
		vowels, last, double := 0, '\u0000', false

		for _, c := range line {
			switch c {
			case 'b':
				if last == 'a' {
					continue outer
				}
			case 'd':
				if last == 'c' {
					continue outer
				}
			case 'q':
				if last == 'p' {
					continue outer
				}
			case 'y':
				if last == 'x' {
					continue outer
				}
			}
			switch c {
			case 'a', 'e', 'i', 'o', 'u':
				vowels++
			}
			if c == last {
				double = true
			}
			last = c
		}
		if double == true && vowels >= 3 {
			nice++
		}
	}
	return nice, nil
}

func part2(input string) (int, error) {
	nice := 0
outer:
	for _, line := range strings.Split(input, "\n") {
		pairs, last, twoAgo := make(map[string][]int), '\u0000', '\u0000'
		split, twoPair, b := false, false, strings.Builder{}

		for i, c := range line {
			b.WriteRune(last)
			b.WriteRune(c)
			pair := b.String()
			b.Reset()
			if _, ok := pairs[pair]; !ok {
				pairs[pair] = make([]int, 0)
			}
			pairs[pair] = append(pairs[pair], i)

			if c == twoAgo {
				split = true
			}
			pairsLen := len(pairs[pair])
			if (pairsLen > 1 && (pairs[pair][pairsLen-1]-pairs[pair][pairsLen-2] > 1)) || pairsLen > 2 {
				twoPair = true
			}
			if split && twoPair {
				nice++
				continue outer
			}
			twoAgo = last
			last = c
		}
	}
	return nice, nil
}
