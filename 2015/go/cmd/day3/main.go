package main

import (
	"log"
	"os"

	"github.com/e3b0c442/adventofcode/2015/go/day3"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("No input path provided")
	}

	if err := day3.Day3(os.Args[1]); err != nil {
		log.Fatal(err)
	}
}
