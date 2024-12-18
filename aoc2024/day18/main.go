package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

type Coords utils.Vec2

func (c *Coords) GetXY() (int, int) {
	return c.X, c.Y
}

func (c *Coords) ToString() string {
	return fmt.Sprintf("%d,%d", c.X, c.Y)
}

type Direction uint8

type Queue utils.Stack[Coords]

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
	}

	panic("unknown direction")
}

func GetDirections() []Direction {
	return []Direction{Up, Right, Down, Left}
}

func NewCoords(x int, y int) Coords {
	return Coords{X: x, Y: y}
}

func (c *Coords) InBounds(w int, h int) bool {
	return c.X >= 0 && c.X <= w && c.Y >= 0 && c.Y <= h
}

func ParseCorruptedBytes(filename string, limit int) map[Coords]bool {
	input := utils.ReadLines(fmt.Sprintf("day18/%s", filename))

	if limit == 0 {
		limit = len(input)
	}

	result := map[Coords]bool{}
	for _, line := range input[:limit] {
		parts := strings.Split(line, ",")
		x := utils.Atoi(parts[0])
		y := utils.Atoi(parts[1])
		c := NewCoords(x, y)
		result[c] = true
	}

	return result
}

func FindClosest(distances map[Coords]int, queue map[Coords]bool) Coords {
	var closest Coords
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

func FindShortestDistance(filename string, simulatedBytes int, width int, height int) int {
	corruptedBytes := ParseCorruptedBytes(filename, simulatedBytes)
	start := NewCoords(0, 0)
	exit := NewCoords(width, height)

	shortest, _ := findShortestDistanceImpl(start, exit, corruptedBytes, width, height)

	return shortest
}

func findShortestDistanceImpl(start Coords, exit Coords, corruptedBytes map[Coords]bool, width int, height int) (int, []Coords) {
	distances := map[Coords]int{}
	previous := map[Coords]Coords{}
	queue := map[Coords]bool{}

	for y := 0; y <= height; y++ {
		for x := 0; x <= width; x++ {
			c := NewCoords(x, y)
			if _, ok := corruptedBytes[c]; !ok {
				queue[c] = true
				distances[c] = math.MaxInt32
			}
		}
	}

	queue[start] = true
	distances[start] = 0

	for len(queue) > 0 {
		closest := FindClosest(distances, queue)
		delete(queue, closest)

		for _, direction := range GetDirections() {
			x, y := closest.GetXY()
			dx, dy := direction.ToDiffs()
			np := NewCoords(x+dx, y+dy)

			if _, ok := queue[np]; ok {
				distance := distances[closest] + 1

				if distance < distances[np] {
					distances[np] = distance
					previous[np] = closest
				}
			}
		}
	}

	path := []Coords{}
	current := exit

	for {
		path = append(path, current)
		if _, ok := previous[current]; !ok {
			break
		}
		current = previous[current]
	}

	return distances[exit], path
}

func FindFirstUnsolvable(filename string, simulatedBytes int, width int, height int) string {
	newCorruptedBytes := []Coords{}
	for _, line := range utils.ReadLines(fmt.Sprintf("day18/%s", filename))[simulatedBytes:] {
		parts := strings.Split(line, ",")
		x := utils.Atoi(parts[0])
		y := utils.Atoi(parts[1])
		c := NewCoords(x, y)
		newCorruptedBytes = append(newCorruptedBytes, c)
	}

	corruptedBytes := ParseCorruptedBytes(filename, simulatedBytes)
	start := NewCoords(0, 0)
	exit := NewCoords(width, height)
	shortest, path := findShortestDistanceImpl(start, exit, corruptedBytes, width, height)

	for _, newCorruptedByte := range newCorruptedBytes {
		corruptedBytes[newCorruptedByte] = true

		if !slices.Contains(path, newCorruptedByte) {
			continue
		}

		shortest, path = findShortestDistanceImpl(start, exit, corruptedBytes, width, height)

		if shortest == math.MaxInt32 {
			return newCorruptedByte.ToString()
		}
	}

	return ""
}

func main() {
	fmt.Println()
	fmt.Println("=== day 18 ===")
	fmt.Println("part 1:", FindShortestDistance("input.txt", 1024, 70, 70))
	fmt.Println("part 2:", FindFirstUnsolvable("input.txt", 1024, 70, 70))
	fmt.Println("==============")
}
