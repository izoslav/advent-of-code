package main

import (
	"fmt"
	"math"
	"slices"

	"github.com/izoslav/aoc2024/utils"
)

type Coords struct {
	X int
	Y int
}

func NewCoords(x int, y int) Coords {
	return Coords{X: x, Y: y}
}

type Position struct {
	Coords    Coords
	Direction Direction
}

func NewPosition(x int, y int, direction Direction) Position {
	return Position{
		Coords:    Coords{X: x, Y: y},
		Direction: direction,
	}
}

func (p *Position) GetXY() (int, int) {
	return p.Coords.X, p.Coords.Y
}

type Direction uint8

const (
	Up Direction = iota
	Right
	Down
	Left
)

func (d Direction) ToDiffs() (int, int) {
	switch d {
	case Up:
		return 0, -1
	case Right:
		return 1, 0
	case Down:
		return 0, 1
	case Left:
		return -1, 0
	default:
		panic("unknown direction")
	}
}

func GetDirections() []Direction {
	return []Direction{
		Up,
		Right,
		Down,
		Left,
	}
}

type Stack []Position

func (s *Stack) Push(p Position) {
	*s = append(*s, p)
}

func (s *Stack) Pop() *Position {
	l := len(*s)
	if l == 0 {
		return nil
	}

	result := (*s)[l-1]
	*s = (*s)[:l-1]

	return &result
}

func ParseMap(filepath string) (start Coords, exit Coords, result map[Coords]bool) {
	lines := utils.ReadLines(filepath)

	result = map[Coords]bool{}

	for y, line := range lines {
		for x, r := range line {
			switch r {
			case '.':
				coords := NewCoords(x, y)
				result[coords] = true
			case 'S':
				start.X = x
				start.Y = y
				result[start] = true
			case 'E':
				exit.X = x
				exit.Y = y
				result[exit] = true
			}
		}
	}

	return start, exit, result
}

func FindClosest(distances map[Position]int, queue map[Position]bool) Position {
	var closest Position
	currentMin := math.MaxInt

	for p, distance := range distances {
		if _, ok := queue[p]; !ok {
			continue
		}

		if distance <= currentMin {
			closest = p
			currentMin = distance
		}
	}

	return closest
}

func FindShortestDistanceAndBestPathsNodes(start Coords, exit Coords, nodes map[Coords]bool) (int, int) {
	distances := map[Position]int{}
	previous := map[Position][]Position{}
	queue := map[Position]bool{}

	for c, _ := range nodes {
		for _, d := range GetDirections() {
			p := NewPosition(c.X, c.Y, d)
			queue[p] = true
			distances[p] = math.MaxInt
		}
	}

	sp := NewPosition(start.X, start.Y, Right)
	queue[sp] = true
	distances[sp] = 0

	for len(queue) > 0 {
		closest := FindClosest(distances, queue)
		delete(queue, closest)

		for _, direction := range GetDirections() {
			x, y := closest.GetXY()
			dx, dy := direction.ToDiffs()
			np := NewPosition(x+dx, y+dy, direction)

			if _, ok := queue[np]; ok {
				// previous distance + 1 step
				distance := distances[closest] + 1

				// +1000 points on turn
				if direction != closest.Direction {
					distance += 1000
				}

				if distance < distances[np] {
					distances[np] = distance
					previous[np] = []Position{closest}
				}

				if distance == distances[np] && !slices.Contains(previous[np], closest) {
					previous[np] = append(previous[np], closest)
				}
			}
		}
	}

	shortest := math.MaxInt
	for _, d := range GetDirections() {
		p := NewPosition(exit.X, exit.Y, d)

		if distance, ok := distances[p]; ok {
			shortest = min(shortest, distance)
		}
	}

	bestPathsNodes := map[Coords]bool{}
	nodeQueue := Stack{}
	for _, d := range GetDirections() {
		p := NewPosition(exit.X, exit.Y, d)

		if distances[p] == shortest {
			nodeQueue.Push(p)
		}
	}

	for len(nodeQueue) > 0 {
		p := nodeQueue.Pop()
		bestPathsNodes[p.Coords] = true

		for _, node := range previous[*p] {
			nodeQueue.Push(node)
		}
	}

	tilesInBestPaths := len(bestPathsNodes)

	return shortest, tilesInBestPaths
}

func main() {
	// filepath := "day16/test.txt"
	filepath := "day16/input.txt"
	start, exit, nodes := ParseMap(filepath)
	shortestDistance, bestPathsNodes := FindShortestDistanceAndBestPathsNodes(start, exit, nodes)

	fmt.Println()
	fmt.Println("=== day 16 ===")
	fmt.Println("part 1:", shortestDistance)
	fmt.Println("part 2:", bestPathsNodes)
	fmt.Println("==============")
}
