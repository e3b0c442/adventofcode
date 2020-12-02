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
	}

	for i, f := range funcs {
		f(fmt.Sprintf("%s/%d.txt", os.Args[1], i+1))
	}
}
