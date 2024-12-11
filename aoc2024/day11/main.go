package main

import (
	"fmt"

	"github.com/izoslav/aoc2024/utils"
)

type Iteration struct {
	n int
	v int
}

func simulate(stones []int, steps int) int {
	result := 0
	memo := map[Iteration]int{}

	for _, stone := range stones {
		result += simulateStep(stone, steps, memo)
	}

	return result
}

func simulateStep(stone int, iterations int, memo map[Iteration]int) int {
	if iterations == 0 {
		return 1
	}

	i := Iteration{n: iterations - 1, v: stone}
	if r, ok := memo[i]; ok {
		return r
	}

	if stone == 0 {
		r := simulateStep(1, iterations-1, memo)
		memo[i] = r
		return r
	}

	s := fmt.Sprintf("%d", stone)
	n := len(s)
	if n%2 == 0 {
		left := utils.Atoi(s[:n/2])
		right := utils.Atoi(s[n/2:])
		r := simulateStep(left, iterations-1, memo) + simulateStep(right, iterations-1, memo)
		memo[i] = r
		return r
	}

	r := simulateStep(stone*2024, iterations-1, memo)
	memo[i] = r
	return r
}

func main() {
	// input := utils.ReadInts("day11/test.txt")
	input := utils.ReadInts("day11/input.txt")

	fmt.Println()
	fmt.Println("=== day 11 ===")
	fmt.Println("part 1:", simulate(input, 25))
	fmt.Println("part 2:", simulate(input, 75))
}
