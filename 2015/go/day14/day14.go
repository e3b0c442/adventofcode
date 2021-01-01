package day14

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

//Day14 Reindeer Olympics
func Day14(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 14: Reindeer Olympics")
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

var reindeerRE = regexp.MustCompile(`([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.`)

type reindeer struct {
	velocity int
	fly      int
	rest     int
}

func parseReindeer(input string) (map[string]reindeer, error) {
	deer := make(map[string]reindeer)
	for _, line := range strings.Split(input, "\n") {
		matches := reindeerRE.FindStringSubmatch(line)
		if len(matches) < 5 {
			return nil, fmt.Errorf("Invalid input: %s", line)
		}

		velocity, err := strconv.Atoi(matches[2])
		if err != nil {
			return nil, err
		}
		fly, err := strconv.Atoi(matches[3])
		if err != nil {
			return nil, err
		}
		rest, err := strconv.Atoi(matches[4])
		if err != nil {
			return nil, err
		}
		deer[matches[1]] = reindeer{
			velocity: velocity,
			fly:      fly,
			rest:     rest,
		}
	}
	return deer, nil
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func part1(input string) (int, error) {
	deer, err := parseReindeer(input)
	if err != nil {
		return 0, err
	}

	dists := make([]int, 0, len(deer))
	for _, r := range deer {
		dists = append(dists, (2503/(r.fly+r.rest))*(r.velocity*r.fly)+min(2503%(r.fly+r.rest), r.fly)*r.velocity)
	}

	sort.Sort(sort.Reverse(sort.IntSlice(dists)))
	return dists[0], nil
}

func part2(input string) (int, error) {
	deer, err := parseReindeer(input)
	if err != nil {
		return 0, err
	}
	scores := make(map[string]int)

	for i := 1; i <= 2503; i++ {
		max := 0
		var winners []string
		for n, r := range deer {
			score := (i/(r.fly+r.rest))*(r.velocity*r.fly) + min(i%(r.fly+r.rest), r.fly)*r.velocity
			if score > max {
				winners = nil
				winners = append(winners, n)
				max = score
			} else if score == max {
				winners = append(winners, n)
			}
		}

		for _, n := range winners {
			scores[n]++
		}
	}

	sc := make([]int, 0, len(scores))
	for _, v := range scores {
		sc = append(sc, v)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(sc)))
	return sc[0], nil
}
