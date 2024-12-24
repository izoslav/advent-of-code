package main

import (
	"fmt"
	"slices"
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

func solve(filepath string) (triplesWithT int, biggestClique int) {
	nodes, connections := parseConnections(filepath)

	nodeList := []string{}
	for k := range nodes {
		nodeList = append(nodeList, k)
	}

	// part 1
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

	triplesWithT = len(triples)

	// part 2
	for _, node := range nodeList {
		biggestClique = max(biggestClique, maxClique(node, []string{}, connections))
	}

	return
}

var cliqueCache = map[string]bool{}

func isClique(nodes []string, connections map[Connection]bool) (result bool) {
	// fmt.Println(nodes)
	if len(nodes) == 1 {
		return true
	}

	slices.Sort(nodes)
	key := strings.Join(nodes, ",")
	if v, ok := cliqueCache[key]; ok {
		return v
	}

	if isClique(nodes[:len(nodes)-1], connections) {
		result = true
		newest := nodes[len(nodes)-1]
		for _, node := range nodes[:len(nodes)-1] {
			if !(connections[Connection{node, newest}] && connections[Connection{newest, node}]) {
				result = false
				break
			}
		}
	}
	cliqueCache[key] = result

	return result
}

var cliqueSizeCache = map[string]int{}
var maxCliqueSize = 0
var maxCliquePassword = ""

func maxClique(newNode string, nodes []string, connections map[Connection]bool) (result int) {
	newNodes := slices.Clone(nodes)
	newNodes = append(newNodes, newNode)
	slices.Sort(newNodes)

	key := strings.Join(newNodes, ",")
	if v, ok := cliqueSizeCache[key]; ok {
		return v
	}

	if !isClique(newNodes, connections) {
		return
	}

	result = len(newNodes)
	for k := range connections {
		if k.a == newNode {
			result = max(result, maxClique(k.b, newNodes, connections))
		}
	}
	cliqueSizeCache[key] = result

	if result > maxCliqueSize {
		maxCliqueSize = result
		maxCliquePassword = strings.Join(newNodes, ",")
	}

	return
}

func main() {
	// tripletsWithT, biggestClique := solve("day23/test.txt")
	tripletsWithT, biggestClique := solve("day23/input.txt")

	fmt.Println()
	fmt.Println("=== day 23 ===")
	fmt.Println("part 1:", tripletsWithT)
	fmt.Println("part 2:", biggestClique, maxCliquePassword)
	fmt.Println("==============")
}
