package main

import (
	"fmt"
	"log"
	"os"

	advent2020 "github.com/e3b0c442/adventofcode/2020/go"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("No input path provided")
	}

	funcs := []func(string) error{
		advent2020.Day1,
		advent2020.Day2,
		advent2020.Day3,
	}

	for i, f := range funcs {
		if err := f(fmt.Sprintf("%s/%d.txt", os.Args[1], i+1)); err != nil {
			log.Fatal(err)
		}
	}
}
