package day12

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

//Day12 JSAbacusFramework.io
func Day12(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 12: JSAbacusFramework.io")
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

func sumJSON(j interface{}, filter string) int {
	sum := 0
	switch v := j.(type) {
	case float64:
		sum += int(v)
	case int:
		sum += v
	case []interface{}:
		for _, vv := range v {
			sum += sumJSON(vv, filter)
		}
	case map[string]interface{}:
		if filter != "" {
			for _, vv := range v {
				if vvv, ok := vv.(string); ok {
					if vvv == filter {
						return 0
					}
				}
			}
		}
		for _, vv := range v {
			sum += sumJSON(vv, filter)
		}
	}
	return sum
}

func part1(input string) (int, error) {
	var js interface{}
	if err := json.Unmarshal([]byte(input), &js); err != nil {
		return 0, err
	}

	return sumJSON(js, ""), nil
}

func part2(input string) (int, error) {
	var js interface{}
	if err := json.Unmarshal([]byte(input), &js); err != nil {
		return 0, err
	}

	return sumJSON(js, "red"), nil
}
