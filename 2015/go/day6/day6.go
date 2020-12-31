package day6

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

//Day6 Probably a Fire Hazard
func Day6(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 6: Probably a Fire Hazard")
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

type instruction struct {
	cmd string
	ox  int
	oy  int
	dx  int
	dy  int
}

var instrRE = regexp.MustCompile(`(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)`)

func parseInstructions(input string) ([]instruction, error) {
	lines := strings.Split(input, "\n")
	instructions := make([]instruction, len(lines))
	for i, line := range lines {
		matches := instrRE.FindStringSubmatch(line)
		if len(matches) < 6 {
			return nil, fmt.Errorf("Invalid input: %s", line)
		}
		instr := instruction{
			cmd: matches[1],
		}
		var err error
		instr.ox, err = strconv.Atoi(matches[2])
		if err != nil {
			return nil, err
		}
		instr.oy, err = strconv.Atoi(matches[3])
		if err != nil {
			return nil, err
		}
		instr.dx, err = strconv.Atoi(matches[4])
		if err != nil {
			return nil, err
		}
		instr.dy, err = strconv.Atoi(matches[5])
		if err != nil {
			return nil, err
		}

		instructions[i] = instr
	}
	return instructions, nil
}

func part1(input string) (int, error) {
	instructions, err := parseInstructions(input)
	if err != nil {
		return 0, err
	}
	grid := make([][]int, 1000)
	for i := range grid {
		grid[i] = make([]int, 1000)
	}

	for _, instr := range instructions {
		switch instr.cmd {
		case "turn off":
			for x := instr.ox; x <= instr.dx; x++ {
				for y := instr.oy; y <= instr.dy; y++ {
					grid[x][y] = 0
				}
			}
		case "turn on":
			for x := instr.ox; x <= instr.dx; x++ {
				for y := instr.oy; y <= instr.dy; y++ {
					grid[x][y] = 1
				}
			}
		case "toggle":
			for x := instr.ox; x <= instr.dx; x++ {
				for y := instr.oy; y <= instr.dy; y++ {
					grid[x][y] ^= 1
				}
			}
		}
	}

	brightness := 0
	for _, row := range grid {
		for _, cell := range row {
			brightness += cell
		}
	}
	return brightness, nil
}

func part2(input string) (int, error) {
	instructions, err := parseInstructions(input)
	if err != nil {
		return 0, err
	}
	grid := make([][]int, 1000)
	for i := range grid {
		grid[i] = make([]int, 1000)
	}

	for _, instr := range instructions {
		switch instr.cmd {
		case "turn off":
			for x := instr.ox; x <= instr.dx; x++ {
				for y := instr.oy; y <= instr.dy; y++ {
					if grid[x][y] > 0 {
						grid[x][y]--
					}
				}
			}
		case "turn on":
			for x := instr.ox; x <= instr.dx; x++ {
				for y := instr.oy; y <= instr.dy; y++ {
					grid[x][y]++
				}
			}
		case "toggle":
			for x := instr.ox; x <= instr.dx; x++ {
				for y := instr.oy; y <= instr.dy; y++ {
					grid[x][y] += 2
				}
			}
		}
	}

	brightness := 0
	for _, row := range grid {
		for _, cell := range row {
			brightness += cell
		}
	}
	return brightness, nil
}
