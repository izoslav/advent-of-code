package main

import (
	"fmt"
	"math"
	"slices"

	"github.com/izoslav/aoc2024/utils"
)

type direction struct {
	x int
	y int
}

var directionMap = map[rune]direction{
	'^': {0, -1},
	'v': {0, 1},
	'<': {-1, 0},
	'>': {1, 0},
}

type index struct {
	x int
	y int
}

var numericKeypad = map[rune]index{
	'7': {0, 0}, '8': {1, 0}, '9': {2, 0},
	'4': {0, 1}, '5': {1, 1}, '6': {2, 1},
	'1': {0, 2}, '2': {1, 2}, '3': {2, 2},
	/*        */ '0': {1, 3}, 'A': {2, 3},
}
var revNumericKeypad = getRevMap(numericKeypad)

var directionKeypad = map[rune]index{
	/*        */ '^': {1, 0}, 'A': {2, 0},
	'<': {0, 1}, 'v': {1, 1}, '>': {2, 1},
}
var revDirectionKeypad = getRevMap(directionKeypad)

var pairsDistanceCache map[string]int
var pathsCache map[string][]string

func getRevMap(m map[rune]index) map[index]rune {
	revMap := map[index]rune{}
	for k, v := range m {
		revMap[v] = k
	}
	return revMap
}

func getTotalComplexity(codes []string, depth int) int {
	complexity := 0
	for _, code := range codes {
		complexity += utils.Atoi(code[:len(code)-1]) * getComplexity("A"+code, depth)
	}
	return complexity
}

func getComplexity(code string, depth int) int {
	cost := 0
	for i := 0; i < len(code)-1; i++ {
		cost += getShortestPath(rune(code[i]), rune(code[i+1]), numericKeypad, revNumericKeypad, depth)
	}
	return cost
}

func getShortestPath(start rune, end rune, keypad map[rune]index, revKeypad map[index]rune, depth int) int {
	keypadType := 'd'
	if _, ok := keypad['0']; ok {
		keypadType = 'n'
	}
	key := fmt.Sprintf("%c%c%c%d", start, end, keypadType, depth)

	if distance, ok := pairsDistanceCache[key]; ok {
		return distance
	}

	if depth == 0 {
		minLen := math.MaxInt
		for _, path := range getPaths(start, end, directionKeypad, revDirectionKeypad) {
			minLen = min(minLen, len(path))
		}
		return minLen
	}

	minCost := math.MaxInt
	paths := getPaths(start, end, keypad, revKeypad)

	for _, path := range paths {
		path = "A" + path
		cost := 0

		for i := 0; i < len(path)-1; i++ {
			cost += getShortestPath(rune(path[i]), rune(path[i+1]), directionKeypad, revDirectionKeypad, depth-1)
		}
		minCost = min(minCost, cost)
	}

	pairsDistanceCache[key] = minCost
	return minCost
}

func getPaths(start rune, end rune, keypad map[rune]index, revKeypad map[index]rune) []string {
	key := fmt.Sprintf("%c %c", start, end)
	if paths, ok := pathsCache[key]; ok {
		return paths
	}

	paths := []string{}
	dfs(keypad[start], keypad[end], []rune{}, keypad, revKeypad, map[index]bool{}, &paths)
	pathsCache[key] = paths

	return paths
}

func dfs(start index, end index, path []rune, keypad map[rune]index, revKeypad map[index]rune, visited map[index]bool, paths *[]string) {
	if start == end {
		*paths = append(*paths, string(path)+"A")
		return
	}

	visited[start] = true
	for c, d := range directionMap {
		nIdx := index{start.x + d.x, start.y + d.y}
		if _, ok := revKeypad[nIdx]; ok && !visited[nIdx] {
			newPath := slices.Clone(path)
			dfs(nIdx, end, append(newPath, c), keypad, revKeypad, visited, paths)
		}
	}
	visited[start] = false
}

func main() {
	pairsDistanceCache = map[string]int{}
	pathsCache = map[string][]string{}

	// codes := utils.ReadLines("day21/test.txt")
	codes := utils.ReadLines("day21/input.txt")

	fmt.Println()
	fmt.Println("=== day 21 ===")
	fmt.Println("part 1:", getTotalComplexity(codes, 2))
	fmt.Println("part 2:", getTotalComplexity(codes, 25))
	fmt.Println("==============")
}
