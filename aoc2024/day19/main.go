package main

import (
	"fmt"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func CountCombinations(towels []string, patterns []string) (int, int) {
	count := 0
	sum := 0
	for _, pattern := range patterns {
		r := countCombinationsStep(towels, pattern, map[string]int{})
		sum += r

		if r > 0 {
			count++
		}
	}
	return count, sum
}

func countCombinationsStep(towels []string, pattern string, cache map[string]int) int {
	if len(pattern) == 0 {
		return 1
	}

	if r, ok := cache[pattern]; ok {
		return r
	}

	result := 0
	for _, towel := range towels {
		if len(towel) > len(pattern) {
			continue
		}

		if strings.HasPrefix(pattern, towel) {
			tail := pattern[len(towel):]
			result += countCombinationsStep(towels, tail, cache)
		}
	}
	cache[pattern] = result

	return result
}

func main() {
	// input := utils.ReadFile("day19/test.txt")
	input := utils.ReadFile("day19/input.txt")
	parts := strings.Split(input, "\n\n")

	towels := strings.Split(parts[0], ", ")
	patterns := strings.Split(parts[1], "\n")

	possiblePatterns, allCombinations := CountCombinations(towels, patterns)

	fmt.Println()
	fmt.Println("=== day 19 ===")
	fmt.Println("part 1:", possiblePatterns)
	fmt.Println("part 2:", allCombinations)
	fmt.Println("==============")
}
