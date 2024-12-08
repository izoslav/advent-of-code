package main

import (
	"fmt"

	"github.com/izoslav/aoc2024/utils"
)

type Point struct {
	x int
	y int
}

func NewPoint(x int, y int) Point {
	return Point{x, y}
}

func (p *Point) InBounds(size int) bool {
	return p.x >= 0 && p.x < size && p.y >= 0 && p.y < size
}

func isAlphanum(c rune) bool {
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')
}

func getAntinodes(antinodes map[Point]bool, p1 Point, p2 Point, size int) {
	dx := p2.x - p1.x
	dy := p2.y - p1.y

	an1 := NewPoint(p1.x-dx, p1.y-dy)
	if an1.InBounds(size) {
		antinodes[an1] = true
	}

	an2 := NewPoint(p2.x+dx, p2.y+dy)
	if an2.InBounds(size) {
		antinodes[an2] = true
	}
}

func getResonantAntinodes(antinodes map[Point]bool, p1 Point, p2 Point, size int) {
	dx := p2.x - p1.x
	dy := p2.y - p1.y

	an := NewPoint(p1.x, p1.y)
	for an.InBounds(size) {
		antinodes[an] = true

		an.x -= dx
		an.y -= dy
	}

	an = NewPoint(p1.x, p1.y)
	for an.InBounds(size) {
		antinodes[an] = true

		an.x += dx
		an.y += dy
	}
}

func main() {
	// input := utils.ReadLines("day08/test.txt")
	input := utils.ReadLines("day08/input.txt")
	size := len(input)

	nodes := map[rune][]Point{}
	for y, line := range input {
		for x, c := range line {
			if isAlphanum(c) {
				nodes[c] = append(nodes[c], NewPoint(x, y))
			}
		}
	}

	antinodes := map[Point]bool{}
	resonantAntinodes := map[Point]bool{}
	for _, nodes := range nodes {
		for i := 0; i < len(nodes)-1; i++ {
			for j := i + 1; j < len(nodes); j++ {
				n1 := nodes[i]
				n2 := nodes[j]

				getAntinodes(antinodes, n1, n2, size)
				getResonantAntinodes(resonantAntinodes, n1, n2, size)
			}
		}
	}

	fmt.Println("part 1:", len(antinodes))
	fmt.Println("part 2:", len(resonantAntinodes))
}
