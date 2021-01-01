package main

import (
	"fmt"
	"log"
	"os"

	"github.com/e3b0c442/adventofcode/2015/go/day1"
	"github.com/e3b0c442/adventofcode/2015/go/day10"
	"github.com/e3b0c442/adventofcode/2015/go/day11"
	"github.com/e3b0c442/adventofcode/2015/go/day12"
	"github.com/e3b0c442/adventofcode/2015/go/day13"
	"github.com/e3b0c442/adventofcode/2015/go/day14"
	"github.com/e3b0c442/adventofcode/2015/go/day2"
	"github.com/e3b0c442/adventofcode/2015/go/day3"
	"github.com/e3b0c442/adventofcode/2015/go/day4"
	"github.com/e3b0c442/adventofcode/2015/go/day5"
	"github.com/e3b0c442/adventofcode/2015/go/day6"
	"github.com/e3b0c442/adventofcode/2015/go/day7"
	"github.com/e3b0c442/adventofcode/2015/go/day8"
	"github.com/e3b0c442/adventofcode/2015/go/day9"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("No input path provided")
	}

	funcs := []func(string) error{
		day1.Day1,
		day2.Day2,
		day3.Day3,
		day4.Day4,
		day5.Day5,
		day6.Day6,
		day7.Day7,
		day8.Day8,
		day9.Day9,
		day10.Day10,
		day11.Day11,
		day12.Day12,
		day13.Day13,
		day14.Day14,
	}

	for i, f := range funcs {
		if err := f(fmt.Sprintf("%s/%d.txt", os.Args[1], i+1)); err != nil {
			log.Fatal(err)
		}
	}
}
