package day9

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

//Day9 All in a Single Night
func Day9(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 9: All in a Single Night")
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

var distRE = regexp.MustCompile(`([A-Za-z]+) to ([A-Za-z]+) = (\d+)`)

func inputToDists(input string) (map[string]map[string]int, error) {
	dists := make(map[string]map[string]int)
	for _, line := range strings.Split(input, "\n") {
		matches := distRE.FindStringSubmatch(line)
		if len(matches) < 4 {
			return nil, fmt.Errorf("Invalid input: %s", line)
		}
		orig, dest := matches[1], matches[2]
		dist, err := strconv.Atoi(matches[3])
		if err != nil {
			return nil, err
		}
		if _, ok := dists[orig]; !ok {
			dists[orig] = make(map[string]int)
		}
		dists[orig][dest] = dist
		if _, ok := dists[dest]; !ok {
			dists[dest] = make(map[string]int)
		}
		dists[dest][orig] = dist
	}

	return dists, nil
}

func makePaths(dists map[string]map[string]int, orig string, remaining map[string]struct{}, dist int) []int {
	if len(remaining) == 0 {
		return []int{dist}
	}

	paths := make([]int, 0)
	for dest := range remaining {
		rm := make(map[string]struct{})
		for k := range remaining {
			if k != dest {
				rm[k] = struct{}{}
			}
		}
		paths = append(paths, makePaths(dists, dest, rm, dist+dists[orig][dest])...)
	}

	return paths
}

func part1(input string) (int, error) {
	dists, err := inputToDists(input)
	if err != nil {
		return 0, err
	}

	paths := make([]int, 0)
	for k := range dists {
		rm := make(map[string]struct{})
		for kk := range dists {
			if k != kk {
				rm[kk] = struct{}{}
			}
		}
		paths = append(paths, makePaths(dists, k, rm, 0)...)
	}

	sort.Ints(paths)
	return paths[0], nil
}

func part2(input string) (int, error) {
	dists, err := inputToDists(input)
	if err != nil {
		return 0, err
	}

	paths := make([]int, 0)
	for k := range dists {
		rm := make(map[string]struct{})
		for kk := range dists {
			if k != kk {
				rm[kk] = struct{}{}
			}
		}
		paths = append(paths, makePaths(dists, k, rm, 0)...)
	}

	sort.Sort(sort.Reverse(sort.IntSlice(paths)))
	return paths[0], nil
}
