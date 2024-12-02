package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func IsLevelSafe(slice []int) bool {
	sign := slice[1] - slice[0]

	if sign == 0 {
		return false
	}

	for i := 0; i < len(slice)-1; i++ {
		diff := slice[i+1] - slice[i]

		if diff*sign <= 0 {
			return false
		}

		diffAbs := utils.AbsInt(diff)

		if diffAbs > 3 {
			return false
		}
	}
	return true
}

func main() {
	// data := utils.ReadLines("day02/test.txt")
	data := utils.ReadLines("day02/input.txt")

	levels := utils.Map[string, []int](data, func(line string) []int {
		input := strings.Split(line, " ")
		return utils.Map[string, int](input, utils.Atoi)
	})

	count := 0
	for _, level := range levels {
		if IsLevelSafe(level) {
			count++
		}
	}

	fmt.Println("part 1:", count)

	safe := 0
	for _, level := range levels {
		if IsLevelSafe(level) {
			safe++
		} else {
			for i := range level {
				newLevel := slices.Concat(level[:i], level[i+1:])

				if IsLevelSafe(newLevel) {
					safe++
					break
				}
			}
		}
	}

	fmt.Println("part 2:", safe)
}
