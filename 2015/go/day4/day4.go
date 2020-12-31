package day4

import (
	"crypto/md5"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

//Day4 The Ideal Stocking Stuffer
func Day4(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 4: The Ideal Stocking Stuffer")
	part1, err := part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := part2(string(input), part1)
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func part1(input string) (int, error) {
	builder := strings.Builder{}
	for i := 1; ; i++ {
		builder.WriteString(input)
		builder.WriteString(strconv.Itoa(i))
		sum := md5.Sum([]byte(builder.String()))
		if sum[0] == 0 && sum[1] == 0 && sum[2]&0xf0 == 0 {
			return i, nil
		}
		builder.Reset()
	}
}

func part2(input string, prev int) (int, error) {
	builder := strings.Builder{}
	for i := prev; ; i++ {
		builder.WriteString(input)
		builder.WriteString(strconv.Itoa(i))
		sum := md5.Sum([]byte(builder.String()))
		if sum[0] == 0 && sum[1] == 0 && sum[2] == 0 {
			return i, nil
		}
		builder.Reset()
	}
}
