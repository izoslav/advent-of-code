package main

import (
	"fmt"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

type Gate struct {
	op string
	r1 string
	r2 string
}

func parseInput(filepath string) (registers map[string]int, gates map[string]Gate) {
	registers = make(map[string]int)
	gates = make(map[string]Gate)

	parts := strings.Split(utils.ReadFile(filepath), "\n\n")

	for _, line := range strings.Split(parts[0], "\n") {
		register := line[:3]
		value := utils.Atoi(line[5:])
		registers[register] = value
	}

	for _, line := range strings.Split(parts[1], "\n") {
		parts := strings.Split(line, " ")
		op := parts[1]
		r1 := parts[0]
		r2 := parts[2]
		or := parts[4]
		gates[or] = Gate{op, r1, r2}
	}

	return
}

func solve(filepath string) (z int) {
	registers, gates := parseInput(filepath)

	for len(gates) > 0 {
		for register, gate := range gates {
			r1, ok1 := registers[gate.r1]
			r2, ok2 := registers[gate.r2]

			if !(ok1 && ok2) {
				continue
			}

			res := 0
			switch gate.op {
			case "AND":
				res = r1 & r2
			case "OR":
				res = r1 | r2
			case "XOR":
				res = r1 ^ r2
			}
			registers[register] = res
			delete(gates, register)
		}
	}

	for register, value := range registers {
		if strings.HasPrefix(register, "z") {
			shift := utils.Atoi(register[1:])
			z |= value << shift
		}
	}

	return
}

func main() {
	// z := solve("day24/test.txt")
	z := solve("day24/input.txt")

	fmt.Println()
	fmt.Println("=== day 24 ===")
	fmt.Println("part 1:", z)
	fmt.Println("part 2: check graphs manually")
	fmt.Println("==============")
}
