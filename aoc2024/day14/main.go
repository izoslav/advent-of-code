package main

import (
	"fmt"
	"os"
	"regexp"

	"github.com/izoslav/aoc2024/utils"
)

type Vec2 struct {
	X int
	Y int
}

type Robot struct {
	Position Vec2
	Velocity Vec2
}

func (r *Robot) Move(times int, width int, height int) {
	dx := r.Velocity.X * times
	dy := r.Velocity.Y * times
	nx := r.Position.X + dx
	ny := r.Position.Y + dy

	if nx < 0 {
		nx = width - (utils.AbsInt(nx) % width)
	}

	if ny < 0 {
		ny = height - (utils.AbsInt(ny) % height)
	}

	r.Position.X = nx % width
	r.Position.Y = ny % height
}

func CalculateSafetyFactor(robots []Robot, seconds int, width int, height int) int {
	mx := width / 2
	my := height / 2

	q1 := 0
	q2 := 0
	q3 := 0
	q4 := 0

	for _, robot := range robots {
		robot.Move(seconds, width, height)

		if robot.Position.X < mx {
			if robot.Position.Y < my {
				q1++
			}

			if robot.Position.Y > my {
				q3++
			}
		}

		if robot.Position.X > mx {
			if robot.Position.Y < my {
				q2++
			}

			if robot.Position.Y > my {
				q4++
			}
		}
	}

	return q1 * q2 * q3 * q4
}

func parseInput(lines []string) (int, int, []Robot) {
	r, _ := regexp.Compile("-?\\d+")

	width := 0
	height := 0
	result := make([]Robot, len(lines))

	for i, line := range lines {
		m := r.FindAllString(line, 4)
		px := utils.Atoi(m[0])
		py := utils.Atoi(m[1])
		vx := utils.Atoi(m[2])
		vy := utils.Atoi(m[3])

		width = max(width, px)
		height = max(height, py)

		result[i] = Robot{
			Position: Vec2{X: px, Y: py},
			Velocity: Vec2{X: vx, Y: vy},
		}
	}

	return width, height, result
}

func drawRobots(robots []Robot, maxSteps int, width int, height int) {
	for i := 0; i < maxSteps; i++ {
		filename := fmt.Sprintf("day14/drawings/%d.txt", i)
		file, err := os.Create(filename)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		for _, robot := range robots {
			robot.Move(i, width, height)
			fmt.Fprintln(file, robot.Position.X, robot.Position.Y)
		}

		file.Close()
	}
}

func main() {
	// input := utils.ReadLines("day14/test.txt")
	input := utils.ReadLines("day14/input.txt")

	width, height, robots := parseInput(input)
	drawRobots(robots, 10000, width+1, height+1)

	fmt.Println()
	fmt.Println("=== day 14 ===")
	fmt.Println("part 1:", CalculateSafetyFactor(robots, 100, width+1, height+1))
	fmt.Println("part 2: run draw.py")
}
