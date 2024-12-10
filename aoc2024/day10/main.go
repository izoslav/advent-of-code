package main

import (
	"fmt"

	"github.com/izoslav/aoc2024/utils"
)

type Point struct {
	x int
	y int
}

var Directions []Point = []Point{
	Point{x: 0, y: -1},
	Point{x: 0, y: 1},
	Point{x: -1, y: 0},
	Point{x: 1, y: 0},
}

type Stack []Point

func (s *Stack) Push(p Point) {
	*s = append(*s, p)
}

func (s *Stack) Pop() *Point {
	l := len(*s)
	if l == 0 {
		return nil
	}

	result := (*s)[l-1]
	*s = (*s)[:l-1]

	return &result
}

func getTrailheads(topographicMap [][]int) []Point {
	trailheads := []Point{}

	for y, row := range topographicMap {
		for x, height := range row {
			if height == 0 {
				trailheads = append(trailheads, Point{x: x, y: y})
			}
		}
	}

	return trailheads
}

func inInBounds(topographicMap [][]int, x int, y int) bool {
	mx := len(topographicMap)
	my := len(topographicMap[0])

	return x >= 0 && x < mx && y >= 0 && y < my
}

func calculateScoresSum(topographicMap [][]int, trailheads []Point) int {
	sum := 0
	for _, trailhead := range trailheads {
		sum += calculateScore(topographicMap, trailhead)
	}

	return sum
}

func calculateScore(topographicMap [][]int, trailhead Point) int {
	frontier := Stack{}
	frontier.Push(trailhead)

	visited := map[Point]bool{}
	score := 0

	for {
		current := frontier.Pop()
		if current == nil {
			break
		}

		if _, ok := visited[*current]; ok {
			continue
		}

		if topographicMap[current.y][current.x] == 9 {
			score++
		}

		for _, direction := range Directions {
			nx := current.x + direction.x
			ny := current.y + direction.y

			if inInBounds(topographicMap, nx, ny) {
				diff := topographicMap[ny][nx] - topographicMap[current.y][current.x]

				if diff == 1 {
					frontier.Push(Point{x: nx, y: ny})
				}
			}
		}

		visited[*current] = true
	}

	return score
}

func calculateRatingSum(topographicMap [][]int, trailheads []Point) int {
	sum := 0
	for _, trailhead := range trailheads {
		sum += calculateRating(topographicMap, trailhead)
	}

	return sum
}

func calculateRating(topographicMap [][]int, trailhead Point) int {
	return calculateRatingStep(topographicMap, trailhead, map[Point]bool{})
}

func calculateRatingStep(topographicMap [][]int, current Point, visited map[Point]bool) int {
	if topographicMap[current.y][current.x] == 9 {
		return 1
	}

	if _, ok := visited[current]; ok {
		return 0
	}

	newVisited := copyMap(visited)
	newVisited[current] = true

	rating := 0
	for _, direction := range Directions {
		nx := current.x + direction.x
		ny := current.y + direction.y

		if inInBounds(topographicMap, nx, ny) {
			diff := topographicMap[ny][nx] - topographicMap[current.y][current.x]

			if diff == 1 {
				np := Point{x: nx, y: ny}
				rating += calculateRatingStep(topographicMap, np, newVisited)
			}
		}
	}

	return rating
}

func copyMap[K comparable, V any](m map[K]V) map[K]V {
	nm := map[K]V{}
	for k, v := range m {
		nm[k] = v
	}
	return nm
}

func main() {
	// input := utils.ReadLines("day10/test.txt")
	input := utils.ReadLines("day10/input.txt")

	topographicMap := make([][]int, len(input))

	for y, line := range input {
		topographicMap[y] = make([]int, len(line))
		for x, rune := range line {
			topographicMap[y][x] = int(rune) - '0'
		}
	}

	trailheads := getTrailheads(topographicMap)

	fmt.Println()
	fmt.Println("=== day 10 ===")
	fmt.Println("part 1:", calculateScoresSum(topographicMap, trailheads))
	fmt.Println("part 2:", calculateRatingSum(topographicMap, trailheads))
}
