package main

import (
	"fmt"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func add(a int64, b int64) int64 {
	return a + b
}

func mul(a int64, b int64) int64 {
	return a * b
}

func concat(a int64, b int64) int64 {
	return utils.Atoi64(fmt.Sprintf("%d%d", a, b))
}

func calibrate(target int64, numbers []int64, operations []func(int64, int64) int64) bool {
	calibrated := false
	for _, op := range operations {
		calibrated = calibrated || _calibrate(target, numbers[0], numbers[1:], operations, op)
	}

	return calibrated
}

func _calibrate(target int64, acc int64, numbers []int64, operations []func(int64, int64) int64, operation func(int64, int64) int64) bool {
	if len(numbers) == 0 {
		return target == acc
	}

	acc = operation(acc, numbers[0])

	calibrated := false
	for _, op := range operations {
		calibrated = calibrated || _calibrate(target, acc, numbers[1:], operations, op)
	}

	return calibrated
}

func main() {
	// input := utils.ReadLines("day07/test.txt")
	input := utils.ReadLines("day07/input.txt")

	equations := map[int64][]int64{}
	for _, line := range input {
		parts := strings.Split(line, ": ")

		testValue := utils.Atoi64(parts[0])

		equations[testValue] = []int64{}

		for _, number := range strings.Split(parts[1], " ") {
			equations[testValue] = append(equations[testValue], utils.Atoi64(number))
		}
	}

	calibrationResultP1 := int64(0)
	calibrationResultP2 := int64(0)
	for testValue, numbers := range equations {
		if calibrate(testValue, numbers, []func(int64, int64) int64{add, mul}) {
			calibrationResultP1 += testValue
		}
		if calibrate(testValue, numbers, []func(int64, int64) int64{add, mul, concat}) {
			calibrationResultP2 += testValue
		}
	}

	fmt.Println("part 1:", calibrationResultP1)
	fmt.Println("part 2:", calibrationResultP2)
}
