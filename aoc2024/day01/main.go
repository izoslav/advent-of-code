package main

import (
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	// data, err := os.ReadFile("day01/test.txt")
	data, err := os.ReadFile("day01/input.txt")
	if err != nil {
		os.Exit(1)
	}

	lines := strings.Split(string(data), "\n")

	left := []int{}
	right := []int{}

	for _, line := range lines {
		entries := strings.Split(line, "   ")

		if len(entries) < 2 {
			continue
		}

		leftEntry, _ := strconv.Atoi(entries[0])
		rightEntry, _ := strconv.Atoi(entries[1])

		left = append(left, leftEntry)
		right = append(right, rightEntry)
	}

	slices.Sort(left)
	slices.Reverse(left)

	slices.Sort(right)
	slices.Reverse(right)

	totalDiff := 0
	for i := 0; i < len(left); i++ {
		diff := left[i] - right[i]
		diff = int(math.Abs(float64(diff)))
		totalDiff += diff
	}

	fmt.Println("part 1:", totalDiff)

	rightMap := map[int]int{}
	for _, entry := range right {
		rightMap[entry]++
	}

	similarityScore := 0
	for _, entry := range left {
		similarityScore += entry * rightMap[entry]
	}

	fmt.Println("part 2:", similarityScore)
}
