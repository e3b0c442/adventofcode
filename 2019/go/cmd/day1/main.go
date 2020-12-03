package main

import (
	"log"
	"os"

	advent2019 "github.com/e3b0c442/adventofcode/2019/go"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("No input path provided")
	}

	advent2019.Day1(os.Args[1])
}
