package main

import (
	"fmt"
	"sort"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func concat(a, b, c, d byte) string {
	return fmt.Sprintf("%c%c%c%c", a, b, c, d)
}

func checkHorizontal(input []string, x int, y int) int {
	count := 0

	if x >= 3 {
		a := input[y][x]
		b := input[y][x-1]
		c := input[y][x-2]
		d := input[y][x-3]

		if concat(a, b, c, d) == "XMAS" {
			count++
		}
	}

	if x < len(input)-3 {
		if input[y][x:x+4] == "XMAS" {
			count++
		}
	}

	return count
}

func checkVertical(input []string, x int, y int) int {
	count := 0

	if y >= 3 {
		a := input[y][x]
		b := input[y-1][x]
		c := input[y-2][x]
		d := input[y-3][x]

		if concat(a, b, c, d) == "XMAS" {
			count++
		}
	}

	if y < len(input)-3 {
		a := input[y][x]
		b := input[y+1][x]
		c := input[y+2][x]
		d := input[y+3][x]

		if concat(a, b, c, d) == "XMAS" {
			count++
		}
	}

	return count
}

func checkDiagonal(input []string, x int, y int) int {
	n := len(input)
	count := 0

	// to the left
	if x >= 3 {
		// left-up
		if y >= 3 {
			a := input[y][x]
			b := input[y-1][x-1]
			c := input[y-2][x-2]
			d := input[y-3][x-3]

			if concat(a, b, c, d) == "XMAS" {
				count++
			}
		}

		// left-down
		if y < n-3 {
			a := input[y][x]
			b := input[y+1][x-1]
			c := input[y+2][x-2]
			d := input[y+3][x-3]

			if concat(a, b, c, d) == "XMAS" {
				count++
			}
		}
	}

	// to the right
	if x < n-3 {
		// right-up
		if y >= 3 {
			a := input[y][x]
			b := input[y-1][x+1]
			c := input[y-2][x+2]
			d := input[y-3][x+3]

			if concat(a, b, c, d) == "XMAS" {
				count++
			}
		}

		// right-down
		if y < n-3 {
			a := input[y][x]
			b := input[y+1][x+1]
			c := input[y+2][x+2]
			d := input[y+3][x+3]

			if concat(a, b, c, d) == "XMAS" {
				count++
			}
		}
	}

	return count
}

func checkWords(input []string) int {
	count := 0

	for y := range input {
		for x := range input[y] {
			if input[y][x] == 'X' {
				count += checkDiagonal(input, x, y)
				count += checkHorizontal(input, x, y)
				count += checkVertical(input, x, y)
			}
		}
	}

	return count
}

func checkXMas(input []string) int {
	n := len(input)
	count := 0

	for y := range input {
		if y < 1 || y > n-2 {
			continue
		}

		for x := range input[y] {
			if x < 1 || x > n-2 {
				continue
			}

			if input[y][x] == 'A' {
				lu := input[y-1][x-1]
				ld := input[y+1][x-1]
				ce := input[y][x]
				ru := input[y-1][x+1]
				rd := input[y+1][x+1]

				letters := fmt.Sprintf("%c%c%c%c%c", lu, ld, ce, ru, rd)
				s := strings.Split(letters, "")
				sort.Strings(s)
				letters = strings.Join(s, "")

				if letters == "AMMSS" && lu != rd {
					count++
				}
			}
		}
	}

	return count
}

func main() {
	// data := utils.ReadLines("day04/test.txt")
	data := utils.ReadLines("day04/input.txt")

	fmt.Println("part 1:", checkWords(data))
	fmt.Println("part 2:", checkXMas(data))
}
