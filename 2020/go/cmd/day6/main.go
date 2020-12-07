package main

import (
	"log"
	"os"

	advent2020 "github.com/e3b0c442/adventofcode/2020/go"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("No input path provided")
	}

	if err := advent2020.Day6(os.Args[1]); err != nil {
		log.Fatal(err)
	}
}
