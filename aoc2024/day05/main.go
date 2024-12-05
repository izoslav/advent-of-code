package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

func parseRules(input string) map[int][]int {
	rules := map[int][]int{}

	for _, line := range strings.Split(input, "\n") {
		pages := strings.Split(line, "|")
		page := utils.Atoi(pages[0])
		required := utils.Atoi(pages[1])

		rules[page] = append(rules[page], required)
	}

	return rules
}

func parsePages(input string) [][]int {
	pagesLines := strings.Split(input, "\n")

	pages := [][]int{}

	for _, pageLine := range pagesLines {
		pagesStr := strings.Split(pageLine, ",")
		pagesEntry := []int{}

		for _, pageStr := range pagesStr {
			pagesEntry = append(pagesEntry, utils.Atoi(pageStr))
		}

		pages = append(pages, pagesEntry)
	}

	return pages
}

func checkOrder(pages []int, requirements map[int][]int) bool {
	for i, page := range pages {
		if _, ok := requirements[page]; !ok {
			continue
		}

		for _, requirement := range requirements[page] {
			if slices.Contains(pages[:i], requirement) && !slices.Contains(pages[i+1:], requirement) {
				return false
			}
		}
	}

	return true
}

func fixOrder(pages []int, requirements map[int][]int) {
	correct := false

	for !correct {
		for i, page := range pages {
			if _, ok := requirements[page]; !ok {
				continue
			}

			for _, requirement := range requirements[page] {
				if slices.Contains(pages[:i], requirement) && !slices.Contains(pages[i+1:], requirement) {
					reqIdx := slices.Index(pages, requirement)
					pages[i], pages[reqIdx] = pages[reqIdx], pages[i]
				}
			}
		}

		correct = checkOrder(pages, requirements)
	}
}

func getMiddlePage(pages []int) int {
	return pages[len(pages)/2]
}

func main() {
	// data := strings.Split(utils.ReadFile("day05/test.txt"), "\n\n")
	data := strings.Split(utils.ReadFile("day05/input.txt"), "\n\n")

	rules := parseRules(data[0])
	pages := parsePages(data[1])

	incorrectlyOrderedPages := [][]int{}
	middlePagesSum := 0
	for _, page := range pages {
		if checkOrder(page, rules) {
			middlePagesSum += getMiddlePage(page)
		} else {
			incorrectlyOrderedPages = append(incorrectlyOrderedPages, page)
		}
	}

	fmt.Println("part 1:", middlePagesSum)

	incorrectOrderSum := 0
	for _, page := range incorrectlyOrderedPages {
		fixOrder(page, rules)
		incorrectOrderSum += getMiddlePage(page)
	}

	fmt.Println("part 2:", incorrectOrderSum)
}
