package day7

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

//Day7 Some Assembly Required
func Day7(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 7: Some Assembly Required")
	part1, err := part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := part2(string(input), part1)
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

var gatesRE = regexp.MustCompile(`(.*) -> ([a-z]+)`)
var gateRE = regexp.MustCompile(`(?:([a-z]+|[0-9]+) )?(?:(AND|OR|NOT|RSHIFT|LSHIFT) )?([a-z]+|[0-9]+)`)

func execGate(gates map[string]string, cache map[string]uint16, key string) (uint16, error) {
	if cached, ok := cache[key]; ok {
		return cached, nil
	}

	if _, ok := gates[key]; !ok {
		return 0, fmt.Errorf("No gate for key %s", key)
	}
	matches := gateRE.FindStringSubmatch(gates[key])
	if len(matches) < 4 {
		return 0, fmt.Errorf("Invalid gate %s", gates[key])
	}
	rr, err := strconv.Atoi(matches[3])
	r := uint16(rr)
	if err != nil {
		r, err = execGate(gates, cache, matches[3])
		if err != nil {
			return 0, err
		}
	}
	if len(matches[2]) == 0 {
		cache[key] = r
		return r, nil
	}

	var l uint16
	if len(matches[1]) > 0 {
		ll, err := strconv.Atoi(matches[1])
		l = uint16(ll)
		if err != nil {
			l, err = execGate(gates, cache, matches[1])
			if err != nil {
				return 0, err
			}
		}
	}
	var rval uint16
	switch matches[2] {
	case "AND":
		rval = l & r
	case "OR":
		rval = l | r
	case "NOT":
		rval = ^r
	case "LSHIFT":
		rval = l << r
	case "RSHIFT":
		rval = l >> r
	default:
		return 0, fmt.Errorf("Invalid op: %s", matches[2])
	}

	cache[key] = rval
	return rval, nil
}

func part1(input string) (int, error) {
	gates := make(map[string]string)
	for _, line := range strings.Split(input, "\n") {
		matches := gatesRE.FindStringSubmatch(line)
		if len(matches) < 3 {
			return 0, fmt.Errorf("Invalid input: %s", line)
		}
		gates[matches[2]] = matches[1]
	}

	cache := make(map[string]uint16)

	rval, err := execGate(gates, cache, "a")
	if err != nil {
		return 0, err
	}
	return int(rval), nil
}

func part2(input string, prev int) (int, error) {
	gates := make(map[string]string)
	for _, line := range strings.Split(input, "\n") {
		matches := gatesRE.FindStringSubmatch(line)
		if len(matches) < 3 {
			return 0, fmt.Errorf("Invalid input: %s", line)
		}
		gates[matches[2]] = matches[1]
	}

	cache := make(map[string]uint16)
	cache["b"] = uint16(prev)

	rval, err := execGate(gates, cache, "a")
	if err != nil {
		return 0, err
	}
	return int(rval), nil
}
