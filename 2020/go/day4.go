package advent2020

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func Day4(inputFile string) error {
	input, err := ioutil.ReadFile(inputFile)
	if err != nil {
		return err
	}

	fmt.Println("Day 4: Passport Processing")
	part1, err := day4Part1(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 1: %d\n", part1)

	part2, err := day4Part2(string(input))
	if err != nil {
		return err
	}
	fmt.Printf("\tPart 2: %d\n", part2)

	return nil
}

func inputToPassports(input string) []map[string]string {
	passports := []map[string]string{}
	records := strings.Split(input, "\n\n")
	for _, record := range records {
		passport := make(map[string]string)
		record = strings.ReplaceAll(record, "\n", " ")
		fields := strings.Split(record, " ")
		for _, field := range fields {
			kv := strings.Split(field, ":")
			passport[kv[0]] = kv[1]
		}
		passports = append(passports, passport)
	}
	return passports
}

func day4Part1(input string) (int, error) {
	passports := inputToPassports(input)
	valid := 0
	for _, pp := range passports {
		if _, ok := pp["byr"]; !ok {
			continue
		}
		if _, ok := pp["iyr"]; !ok {
			continue
		}
		if _, ok := pp["eyr"]; !ok {
			continue
		}
		if _, ok := pp["hgt"]; !ok {
			continue
		}
		if _, ok := pp["hcl"]; !ok {
			continue
		}
		if _, ok := pp["ecl"]; !ok {
			continue
		}
		if _, ok := pp["pid"]; !ok {
			continue
		}
		valid++
	}
	return valid, nil
}

func validatePassport(passport map[string]string) bool {
	if byr, ok := passport["byr"]; ok {
		if len(byr) != 4 {
			return false
		}
		byri, err := strconv.Atoi(byr)
		if err != nil {
			return false
		}
		if byri < 1920 || byri > 2002 {
			return false
		}
	} else {
		return false
	}
	if iyr, ok := passport["iyr"]; ok {
		if len(iyr) != 4 {
			return false
		}
		iyri, err := strconv.Atoi(iyr)
		if err != nil {
			return false
		}
		if iyri < 2010 || iyri > 2020 {
			return false
		}
	} else {
		return false
	}
	if eyr, ok := passport["eyr"]; ok {
		if len(eyr) != 4 {
			return false
		}
		eyri, err := strconv.Atoi(eyr)
		if err != nil {
			return false
		}
		if eyri < 2020 || eyri > 2030 {
			return false
		}
	} else {
		return false
	}
	if hgt, ok := passport["hgt"]; ok {
		u := hgt[len(hgt)-2:]
		vv := hgt[:len(hgt)-2]
		v, err := strconv.Atoi(vv)
		if err != nil {
			return false
		}
		switch u {
		case "cm":
			if v < 150 || v > 193 {
				return false
			}
		case "in":
			if v < 59 || v > 76 {
				return false
			}
		default:
			return false
		}
	} else {
		return false
	}
	if hcl, ok := passport["hcl"]; ok {
		if hcl[0] != '#' {
			return false
		}
		_, err := strconv.ParseInt(hcl[1:], 16, 32)
		if err != nil {
			return false
		}
	} else {
		return false
	}
	if ecl, ok := passport["ecl"]; ok {
		colors := map[string]struct{}{
			"amb": {},
			"blu": {},
			"brn": {},
			"gry": {},
			"grn": {},
			"hzl": {},
			"oth": {},
		}
		if _, ok := colors[ecl]; !ok {
			return false
		}
	} else {
		return false
	}
	if pid, ok := passport["pid"]; ok {
		if len(pid) != 9 {
			return false
		}
		_, err := strconv.Atoi(pid)
		if err != nil {
			return false
		}
	} else {
		return false
	}
	return true
}

func day4Part2(input string) (int, error) {
	passports := inputToPassports(input)
	valid := 0
	for _, passport := range passports {
		if validatePassport(passport) {
			valid++
		}
	}

	return valid, nil
}
