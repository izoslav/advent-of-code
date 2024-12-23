package main

import (
	"fmt"
	"strings"

	"github.com/izoslav/aoc2024/utils"
	"gonum.org/v1/gonum/stat/combin"
)

type Connection struct {
	a string
	b string
}

func parseConnections(filepath string) (nodes map[string]bool, connections map[Connection]bool) {
	connections = make(map[Connection]bool)

	lines := utils.ReadLines(filepath)

	for _, line := range lines {
		nodes := strings.Split(line, "-")
		connections[Connection{nodes[0], nodes[1]}] = true
		connections[Connection{nodes[1], nodes[0]}] = true
	}

	nodes = map[string]bool{}
	for k := range connections {
		nodes[k.a] = true
		nodes[k.b] = true
	}

	return
}

func solve(filepath string) (result int) {
	nodes, connections := parseConnections(filepath)

	nodeList := []string{}
	for k := range nodes {
		nodeList = append(nodeList, k)
	}

	triples := map[string]bool{}

	for _, combination := range combin.Combinations(len(nodes), 3) {
		a := nodeList[combination[0]]
		b := nodeList[combination[1]]
		c := nodeList[combination[2]]

		if !(strings.HasPrefix(a, "t") || strings.HasPrefix(b, "t") || strings.HasPrefix(c, "t")) {
			continue
		}

		_, ab := connections[Connection{a, b}]
		_, ac := connections[Connection{a, c}]
		_, ba := connections[Connection{b, a}]
		_, bc := connections[Connection{b, c}]
		_, ca := connections[Connection{c, a}]
		_, cb := connections[Connection{c, b}]

		if ab && ac && ba && bc && ca && cb {
			triple := []string{a, b, c}
			triples[strings.Join(triple, "")] = true
		}
	}

	result = len(triples)

	return
}

func main() {

	fmt.Println()
	fmt.Println("=== day 23 ===")
	fmt.Println("part 1:", solve("day23/input.txt"))
	fmt.Println("part 2:")
	fmt.Println("==============")
}
