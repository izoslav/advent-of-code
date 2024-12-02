package main

import (
	"fmt"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func IsAscending(slice []int) bool {
	for i := 0; i < len(slice)-1; i++ {
		if slice[i+1] <= slice[i] {
			return false
		}
	}
	return true
}

func IsDescending(slice []int) bool {
	for i := 0; i < len(slice)-1; i++ {
		if slice[i+1] >= slice[i] {
			return false
		}
	}
	return true
}

func IsDiffLessThan3(slice []int) bool {
	for i := 0; i < len(slice)-1; i++ {
		if utils.AbsInt(slice[i+1]-slice[i]) > 3 {
			return false
		}
	}
	return true
}

func IsLevelSafe(slice []int) bool {
	return (IsAscending(slice) || IsDescending(slice)) && IsDiffLessThan3(slice)
}

func removeAt(slice []int, idx int) []int {
	newSlice := make([]int, len(slice)-1)
	for i := 0; i < idx; i++ {
		newSlice[i] = slice[i]
	}

	for i := idx + 1; i < len(slice); i++ {
		newSlice[i-1] = slice[i]
	}
	return newSlice
}

func main() {
	// data := utils.ReadLines("day02/test.txt")
	data := utils.ReadLines("day02/input.txt")

	levels := utils.Map[[]string, string, [][]int, []int](data[:len(data)-1], func(line string) []int {
		input := strings.Split(line, " ")
		return utils.Map[[]string, string, []int, int](input, utils.Atoi)
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
				newLevel := removeAt(level, i)

				if IsLevelSafe(newLevel) {
					safe++
					break
				}
			}
		}
	}

	fmt.Println("part 2:", safe)
}
