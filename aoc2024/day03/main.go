package main

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func main() {
	// data := utils.ReadFile("day03/test.txt")
	data := utils.ReadFile("day03/input.txt")

	r, _ := regexp.Compile(`mul\((\d+),(\d+)\)`)

	result := 0
	for _, match := range r.FindAllStringSubmatch(data, 1000) {
		a := utils.Atoi(match[1])
		b := utils.Atoi(match[2])
		result += a * b
	}

	fmt.Println("part 1:", result)

	result2 := 0
	enabled := true
	r2, _ := regexp.Compile(`(mul\((\d+),(\d+)\))|(do(n't)?\(\))`)
	for _, match2 := range r2.FindAllStringSubmatch(data, 1000) {
		command := match2[0]

		if enabled && strings.Contains(command, "mul") {
			a := utils.Atoi(match2[2])
			b := utils.Atoi(match2[3])
			result2 += a * b
		}

		if command == "do()" {
			enabled = true
		}

		if command == "don't()" {
			enabled = false
		}
	}

	fmt.Println("part 2:", result2)
}
