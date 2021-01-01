package day11

import (
	"fmt"
	"io/ioutil"
)

//Day11 Corporate Policy
func Day11(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 11: Corporate Policy")
	part1, err := part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %s\n", part1)

	part2, err := part2(part1)
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %s\n", part2)

	return nil
}

func checkPassword(password string) bool {
	oneAgo, twoAgo, threeAgo := '\u0000', '\u0000', '\u0000'
	pairs, triplet := 0, false
	for _, c := range password {
		switch c {
		case 'i', 'o', 'l':
			return false
		}
		if c-oneAgo == 1 && oneAgo-twoAgo == 1 {
			triplet = true
		}
		if c == oneAgo && (c != twoAgo || (twoAgo == threeAgo)) {
			pairs++
		}
		if triplet && pairs >= 2 {
			return true
		}
		threeAgo = twoAgo
		twoAgo = oneAgo
		oneAgo = c
	}
	return false
}

func incrementPassword(password string) string {
	for i := len(password) - 1; i >= 0; i-- {
		c := password[i]
		c++
		if c > 'z' {
			c = 'a'
		}
		newPassword := password[0:i] + string(c)
		if i < len(password)-1 {
			newPassword += password[i+1:]
		}
		if c != 'a' {
			return newPassword
		}
		password = newPassword
	}
	return password
}

func part1(input string) (string, error) {
	for !(checkPassword(input)) {
		input = incrementPassword(input)
	}
	return input, nil
}

func part2(input string) (string, error) {
	input = incrementPassword(input)
	for !(checkPassword(input)) {
		input = incrementPassword(input)
	}
	return input, nil
}
