package main

import (
	"fmt"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func parseInput(filepath string) (keys [][]int, holes [][]int) {
	input := utils.ReadFile(filepath)
	entries := strings.Split(input, "\n\n")

	keys = [][]int{}
	holes = [][]int{}

	for _, entry := range entries {
		parsed := make([]int, 5)
		rows := strings.Split(entry, "\n")

		for _, row := range rows[1 : len(rows)-1] {
			for x, c := range row {
				if c == '#' {
					parsed[x]++
				}
			}
		}

		if entry[0] == '.' {
			keys = append(keys, parsed)
		} else {
			holes = append(holes, parsed)
		}
	}

	return
}

func isPair(key []int, hole []int) bool {
	for i := range key {
		if key[i]+hole[i] > 5 {
			return false
		}
	}
	return true
}

func solve(filepath string) (pairs int) {
	keys, holes := parseInput(filepath)

	for _, key := range keys {
		for _, hole := range holes {
			if isPair(key, hole) {
				pairs++
			}
		}
	}

	return
}

func main() {
	fmt.Println()
	fmt.Println("=== day 25 ===")
	fmt.Println("part 1:", solve("day25/input.txt"))
	fmt.Println("part 2: AoC complete!")
	fmt.Println("==============")
}
