package main

import (
	"fmt"

	"github.com/izoslav/aoc2024/utils"
)

type Point struct {
	x int
	y int
}

type Fences struct {
	north bool
	south bool
	east  bool
	west  bool
}

type Garden struct {
	label           rune
	area            int
	fences          int
	price           int
	sides           int
	discountedPrice int
	plots           map[Point]Fences
}

func NewGarden(farmMap [][]rune, start Point) Garden {
	label := farmMap[start.y][start.x]
	plots := map[Point]Fences{}

	fences := countFences(farmMap, plots, label, start)
	area := len(plots)
	price := fences * area

	plotFences := make(map[Point]Fences, len(plots))
	for k, v := range plots {
		plotFences[k] = v
	}

	sides := countSides(plotFences)
	discountedPrice := area * sides

	return Garden{
		label:           label,
		area:            area,
		fences:          fences,
		price:           price,
		sides:           sides,
		discountedPrice: discountedPrice,
		plots:           plots,
	}
}

func countFences(farmMap [][]rune, visited map[Point]Fences, label rune, start Point) int {
	if _, ok := visited[start]; ok {
		return 0
	}

	if farmMap[start.y][start.x] != label {
		return 0
	} else {
		visited[start] = Fences{}
	}

	entry, _ := visited[start]

	fences := 0

	// up
	if start.y-1 >= 0 {
		p := Point{x: start.x, y: start.y - 1}

		if farmMap[p.y][p.x] == label {
			fences += countFences(farmMap, visited, label, p)
		} else {
			entry.north = true
			fences++
		}
	} else {
		entry.north = true
		fences++
	}

	// down
	if start.y+1 < len(farmMap) {
		p := Point{x: start.x, y: start.y + 1}

		if farmMap[p.y][p.x] == label {
			fences += countFences(farmMap, visited, label, p)
		} else {
			entry.south = true
			fences++
		}
	} else {
		entry.south = true
		fences++
	}

	// left
	if start.x-1 >= 0 {
		p := Point{x: start.x - 1, y: start.y}

		if farmMap[p.y][p.x] == label {
			fences += countFences(farmMap, visited, label, p)
		} else {
			entry.west = true
			fences++
		}
	} else {
		entry.west = true
		fences++
	}

	// right
	if start.x+1 < len(farmMap[0]) {
		p := Point{x: start.x + 1, y: start.y}

		if farmMap[p.y][p.x] == label {
			fences += countFences(farmMap, visited, label, p)
		} else {
			entry.east = true
			fences++
		}
	} else {
		entry.east = true
		fences++
	}

	visited[start] = entry

	return fences
}

func countSides(fences map[Point]Fences) int {
	sides := 0
	for p, f := range fences {
		// march up-down for east
		if f.east {
			sides++

			f.east = false

			// march up
			np := Point{x: p.x, y: p.y - 1}
			nf, ok := fences[np]

			for ok && nf.east {
				nf.east = false
				fences[np] = nf
				np = Point{x: np.x, y: np.y - 1}
				nf, ok = fences[np]
			}

			// march down
			np = Point{x: p.x, y: p.y + 1}
			nf, ok = fences[np]

			for ok && nf.east {
				nf.east = false
				fences[np] = nf
				np = Point{x: np.x, y: np.y + 1}
				nf, ok = fences[np]
			}
		}

		// march up-down for west
		if f.west {
			sides++

			f.west = false

			// march up
			np := Point{x: p.x, y: p.y - 1}
			nf, ok := fences[np]

			for ok && nf.west {
				nf.west = false
				fences[np] = nf
				np = Point{x: np.x, y: np.y - 1}
				nf, ok = fences[np]
			}

			// march down
			np = Point{x: p.x, y: p.y + 1}
			nf, ok = fences[np]

			for ok && nf.west {
				nf.west = false
				fences[np] = nf
				np = Point{x: np.x, y: np.y + 1}
				nf, ok = fences[np]
			}
		}

		// march left-right for north
		if f.north {
			sides++

			f.north = false

			// march left
			np := Point{x: p.x - 1, y: p.y}
			nf, ok := fences[np]

			for ok && nf.north {
				nf.north = false
				fences[np] = nf
				np = Point{x: np.x - 1, y: np.y}
				nf, ok = fences[np]
			}

			// march right
			np = Point{x: p.x + 1, y: p.y}
			nf, ok = fences[np]

			for ok && nf.north {
				nf.north = false
				fences[np] = nf
				np = Point{x: np.x + 1, y: np.y}
				nf, ok = fences[np]
			}
		}

		// march left-right for south
		if f.south {
			sides++

			f.south = false

			// march left
			np := Point{x: p.x - 1, y: p.y}
			nf, ok := fences[np]

			for ok && nf.south {
				nf.south = false
				fences[np] = nf
				np = Point{x: np.x - 1, y: np.y}
				nf, ok = fences[np]
			}

			// march right
			np = Point{x: p.x + 1, y: p.y}
			nf, ok = fences[np]

			for ok && nf.south {
				nf.south = false
				fences[np] = nf
				np = Point{x: np.x + 1, y: np.y}
				nf, ok = fences[np]
			}
		}
	}

	return sides
}

func getGardens(farmMap [][]rune) []Garden {
	visited := map[Point]bool{}
	gardenPlots := []Garden{}

	for y := range farmMap {
		for x := range farmMap[0] {
			p := Point{x: x, y: y}

			if _, ok := visited[p]; ok {
				continue
			}

			garden := NewGarden(farmMap, p)

			for k, _ := range garden.plots {
				visited[k] = true
			}

			gardenPlots = append(gardenPlots, garden)
		}
	}

	return gardenPlots
}

func getTotalPrice(gardens []Garden) int {
	sum := 0
	for _, garden := range gardens {
		sum += garden.price
	}
	return sum
}

func getTotalDiscountedPrice(gardens []Garden) int {
	sum := 0
	for _, garden := range gardens {
		sum += garden.discountedPrice
	}
	return sum
}

func main() {
	// input := utils.ReadLines("day12/test.txt")
	input := utils.ReadLines("day12/input.txt")

	farmMap := make([][]rune, len(input))
	for y, line := range input {
		farmMap[y] = make([]rune, len(line))
		for x, r := range line {
			farmMap[y][x] = r
		}
	}

	gardens := getGardens(farmMap)

	fmt.Println()
	fmt.Println("=== day 12 ===")
	fmt.Println("part 1:", getTotalPrice(gardens))
	fmt.Println("part 2:", getTotalDiscountedPrice(gardens))
}
