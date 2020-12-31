package day3

import (
	"fmt"
	"io/ioutil"
)

//Day3 Perfectly Spherical Houses in a Vacuum
func Day3(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 3: Perfectly Spherical Houses in a Vacuum")
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
	x, y, minX, maxX, minY, maxY := 0, 0, 0, 0, 0, 0
	for _, c := range input {
		switch c {
		case '^':
			y++
			if y > maxY {
				maxY = y
			}
		case '>':
			x++
			if x > maxX {
				maxX = x
			}
		case 'v':
			y--
			if y < minY {
				minY = y
			}
		case '<':
			x--
			if x < minX {
				minX = x
			}
		}
	}

	w, h := maxX-minX+1, maxY-minY+1
	grid := make([][]bool, w)
	for i := range grid {
		grid[i] = make([]bool, h)
	}

	x, y = -minX, -minY
	grid[x][y] = true
	for _, c := range input {
		switch c {
		case '^':
			y++
		case '>':
			x++
		case 'v':
			y--
		case '<':
			x--
		}
		grid[x][y] = true
	}

	houses := 0
	for _, row := range grid {
		for _, cell := range row {
			if cell {
				houses++
			}
		}
	}

	return houses, nil

}

func part2(input string) (int, error) {
	x, y, minX, maxX, minY, maxY, s := []int{0, 0}, []int{0, 0}, 0, 0, 0, 0, 0
	for _, c := range input {
		switch c {
		case '^':
			y[s]++
			if y[s] > maxY {
				maxY = y[s]
			}
		case '>':
			x[s]++
			if x[s] > maxX {
				maxX = x[s]
			}
		case 'v':
			y[s]--
			if y[s] < minY {
				minY = y[s]
			}
		case '<':
			x[s]--
			if x[s] < minX {
				minX = x[s]
			}
		}
		s ^= 1
	}

	w, h := maxX-minX+1, maxY-minY+1
	grid := make([][]bool, w)
	for i := range grid {
		grid[i] = make([]bool, h)
	}

	x[0], x[1], y[0], y[1] = -minX, -minX, -minY, -minY
	grid[x[0]][y[0]] = true
	s = 0
	for _, c := range input {
		switch c {
		case '^':
			y[s]++
		case '>':
			x[s]++
		case 'v':
			y[s]--
		case '<':
			x[s]--
		}
		grid[x[s]][y[s]] = true
		s ^= 1
	}

	houses := 0
	for _, row := range grid {
		for _, cell := range row {
			if cell {
				houses++
			}
		}
	}

	return houses, nil
}
