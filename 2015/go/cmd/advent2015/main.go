package main

import (
	"fmt"
	"log"
	"os"

	"github.com/e3b0c442/adventofcode/2015/go/day1"
	"github.com/e3b0c442/adventofcode/2015/go/day2"
	"github.com/e3b0c442/adventofcode/2015/go/day3"
	"github.com/e3b0c442/adventofcode/2015/go/day4"
	"github.com/e3b0c442/adventofcode/2015/go/day5"
	"github.com/e3b0c442/adventofcode/2015/go/day6"
	"github.com/e3b0c442/adventofcode/2015/go/day7"
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
	}

	for i, f := range funcs {
		if err := f(fmt.Sprintf("%s/%d.txt", os.Args[1], i+1)); err != nil {
			log.Fatal(err)
		}
	}
}
