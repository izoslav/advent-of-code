package main

import (
	"fmt"
	"math"
	"regexp"
	"strings"

	"gonum.org/v1/gonum/mat"

	"github.com/izoslav/aoc2024/utils"
)

type Button struct {
	X int
	Y int
}

type Prize struct {
	X int
	Y int
}

type Machine struct {
	A     Button
	B     Button
	Prize Prize
}

type Point struct {
	X int
	Y int
}

func (m *Machine) Simulate(pressLimit int) int {
	memo := map[Point]int{}
	result := m.simulateStep(pressLimit, 0, 0, 0, 0, 0, memo)

	if result == math.MaxInt {
		return 0
	}

	return result
}

func (m *Machine) simulateStep(pressLimit int, x int, y int, a int, b int, tokens int, memo map[Point]int) int {
	if r, ok := memo[Point{X: x, Y: y}]; ok {
		return r
	}

	if x > m.Prize.X || y > m.Prize.Y {
		return math.MaxInt
	}

	if pressLimit != 0 {
		if a > pressLimit || b > pressLimit {
			return math.MaxInt
		}
	}

	var result int

	if x == m.Prize.X && y == m.Prize.Y {
		result = tokens
	} else {
		ar := m.simulateStep(pressLimit, x+m.A.X, y+m.A.Y, a+1, b, tokens+3, memo)
		br := m.simulateStep(pressLimit, x+m.B.X, y+m.B.Y, a, b+1, tokens+1, memo)

		result = min(ar, br)
	}

	memo[Point{X: x, Y: y}] = result

	return result
}

func (m *Machine) MovePrize(x int, y int) {
	m.Prize.X += x
	m.Prize.Y += y
}

func (m *Machine) Simulate2() int {
	A := mat.NewDense(2, 2, []float64{float64(m.A.X), float64(m.B.X), float64(m.A.Y), float64(m.B.Y)})
	B := mat.NewVecDense(2, []float64{float64(m.Prize.X), float64(m.Prize.Y)})

	var x mat.VecDense
	if err := x.SolveVec(A, B); err != nil {
		fmt.Println(err)
		return 0
	}

	a := int(math.Round(x.At(0, 0)))
	b := int(math.Round(x.At(1, 0)))
	ax := m.A.X * a
	ay := m.A.Y * a
	bx := m.B.X * b
	by := m.B.Y * b

	if ax+bx == m.Prize.X && ay+by == m.Prize.Y {
		return a*3 + b
	}

	return 0
}

func parseInput(input string) []Machine {
	r, _ := regexp.Compile("\\d+")

	entries := strings.Split(input, "\n\n")

	result := make([]Machine, len(entries))
	for i, entry := range entries {
		matches := r.FindAllString(entry, 6)

		result[i] = Machine{
			A:     Button{X: utils.Atoi(matches[0]), Y: utils.Atoi(matches[1])},
			B:     Button{X: utils.Atoi(matches[2]), Y: utils.Atoi(matches[3])},
			Prize: Prize{X: utils.Atoi(matches[4]), Y: utils.Atoi(matches[5])},
		}
	}

	return result
}

func main() {
	// input := utils.ReadFile("day13/test.txt")
	input := utils.ReadFile("day13/input.txt")

	machines := parseInput(input)

	p1 := 0
	for _, machine := range machines {
		p1 += machine.Simulate(100)
	}

	p2 := 0
	for _, machine := range machines {
		machine.MovePrize(10000000000000, 10000000000000)
		p2 += machine.Simulate2()
	}

	fmt.Println()
	fmt.Println("=== day 13 ===")
	fmt.Println("part 1:", p1)
	fmt.Println("part 2:", p2)
}
