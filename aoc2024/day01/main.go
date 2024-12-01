package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func main() {
	lines := utils.ReadLines("day01/test.txt")

	left := []int{}
	right := []int{}

	for _, line := range lines {
		entries := strings.Split(line, "   ")

		if len(entries) < 2 {
			continue
		}

		leftEntry := utils.Atoi(entries[0])
		rightEntry := utils.Atoi(entries[1])

		left = append(left, leftEntry)
		right = append(right, rightEntry)
	}

	slices.Sort(left)
	slices.Sort(right)

	zipped := utils.Zip(left, right)
	totalDiff := utils.Fold(zipped, 0, func(acc int, next utils.Pair[int]) int {
		return acc + utils.AbsInt(next.Left-next.Right)
	})

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
