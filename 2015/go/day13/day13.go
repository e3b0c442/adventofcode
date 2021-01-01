package day13

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

//Day13 Knights of the Dinner Table
func Day13(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 13: Knights of the Dinner Table")
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

var knightRE = regexp.MustCompile(`([A-Za-z]+) would (gain|lose) (\d+) happiness units by sitting next to ([A-Za-z]+).`)

func parseSeating(input string) (map[string]map[string]int, error) {
	seating := make(map[string]map[string]int)
	for _, line := range strings.Split(input, "\n") {
		matches := knightRE.FindStringSubmatch(line)
		if len(matches) < 5 {
			return nil, fmt.Errorf("Invalid input: %s", line)
		}
		happiness, err := strconv.Atoi(matches[3])
		if err != nil {
			return nil, err
		}
		switch matches[2] {
		case "gain":
		case "lose":
			happiness = -happiness
		default:
			return nil, fmt.Errorf("Invalid input: %s", line)
		}

		if _, ok := seating[matches[1]]; !ok {
			seating[matches[1]] = make(map[string]int)
		}
		seating[matches[1]][matches[4]] = happiness
	}

	return seating, nil
}

func happiness(likes map[string]map[string]int, people []string, remaining map[string]struct{}, total int) []int {
	if len(remaining) == 0 {
		return []int{total + likes[people[0]][people[len(people)-1]] + likes[people[len(people)-1]][people[0]]}
	}

	happys := make([]int, 0)
	for k := range remaining {
		rm := make(map[string]struct{})
		for kk := range remaining {
			if k != kk {
				rm[kk] = struct{}{}
			}
		}
		pp := make([]string, len(people))
		copy(pp, people)
		pp = append(pp, k)
		happys = append(happys, happiness(likes, pp, rm, total+likes[k][people[len(people)-1]]+likes[people[len(people)-1]][k])...)
	}

	return happys
}

func part1(input string) (int, error) {
	likes, err := parseSeating(input)
	if err != nil {
		return 0, err
	}

	happys := make([]int, 0)
	for k := range likes {
		rm := make(map[string]struct{})
		for kk := range likes {
			if kk != k {
				rm[kk] = struct{}{}
			}
		}
		happys = append(happys, happiness(likes, []string{k}, rm, 0)...)
	}

	sort.Sort(sort.Reverse(sort.IntSlice(happys)))
	return happys[0], nil
}

func part2(input string) (int, error) {
	likes, err := parseSeating(input)
	if err != nil {
		return 0, err
	}

	me := make(map[string]int)
	for k := range likes {
		likes[k]["me"] = 0
		me[k] = 0
	}
	likes["me"] = me

	happys := make([]int, 0)
	for k := range likes {
		rm := make(map[string]struct{})
		for kk := range likes {
			if kk != k {
				rm[kk] = struct{}{}
			}
		}
		happys = append(happys, happiness(likes, []string{k}, rm, 0)...)
	}

	sort.Sort(sort.Reverse(sort.IntSlice(happys)))
	return happys[0], nil
}
