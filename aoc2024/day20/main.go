package main

import (
	"fmt"
	"math"
	"slices"

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

func Distance(a Coords, b Coords) int {
	x1, y1 := a.GetXY()
	x2, y2 := b.GetXY()

	return utils.AbsInt(x2-x1) + utils.AbsInt(y2-y1)
}

func ParseMap(filepath string) (Coords, Coords, map[Coords]bool) {
	input := utils.ReadLines(filepath)

	var start Coords
	var end Coords

	tiles := map[Coords]bool{}
	for y, row := range input {
		for x, r := range row {
			if r != '#' {
				c := NewCoords(x, y)
				tiles[c] = true
			}

			if r == 'S' {
				start = NewCoords(x, y)
			}

			if r == 'E' {
				end = NewCoords(x, y)
			}
		}
	}

	return start, end, tiles
}

func RunWithCheats(start Coords, end Coords, tiles map[Coords]bool, picoseconds int, save int) int {
	_, distances, path := GetDistances(start, end, tiles)
	slices.Reverse(path)

	shortened := 0
	for i, x := range path[:len(path)-picoseconds] {
		for _, y := range path[i+picoseconds:] {
			if distances[y]-distances[x]-Distance(x, y) >= save && Distance(x, y) <= picoseconds {
				shortened++
			}
		}
	}

	return shortened
}

func GetDistances(start Coords, end Coords, tiles map[Coords]bool) (int, map[Coords]int, []Coords) {
	distances := map[Coords]int{}
	previous := map[Coords]Coords{}
	queue := map[Coords]bool{}

	for k := range tiles {
		distances[k] = math.MaxInt32
		queue[k] = true
	}
	distances[start] = 0

	for len(queue) > 0 {
		closest := getClosest(distances, queue)
		delete(queue, closest)

		for _, d := range GetDirections() {
			dx, dy := d.ToDiffs()
			nc := NewCoords(closest.X+dx, closest.Y+dy)
			distance := distances[closest] + 1

			if _, ok := queue[nc]; ok {
				distances[nc] = min(distance, distances[nc])
				previous[nc] = closest
			}
		}
	}

	path := []Coords{}
	current := end
	for current != start {
		path = append(path, current)
		current = previous[current]
	}
	path = append(path, current)

	return distances[end], distances, path
}

func getClosest(distances map[Coords]int, queue map[Coords]bool) Coords {
	var closest Coords
	currentMin := math.MaxInt32

	for p, distance := range distances {
		if _, ok := queue[p]; !ok {
			continue
		}

		if distance < currentMin {
			closest = p
			currentMin = distance
		}
	}

	return closest
}

func main() {
	start, end, tiles := ParseMap("day20/input.txt")

	fmt.Println()
	fmt.Println("=== day 20 ===")
	fmt.Println("part 1:", RunWithCheats(start, end, tiles, 2, 100))
	fmt.Println("part 2:", RunWithCheats(start, end, tiles, 20, 100))
	fmt.Println("==============")
}
