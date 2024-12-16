package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

type Element uint8

const (
	Empty Element = iota
	Box
	WBoxL
	WBoxR
	Wall
	RobotSymbol
)

func (e Element) IsBox() bool {
	return e == Box || e == WBoxL || e == WBoxR
}

func ElementFromByte(r byte) Element {
	switch r {
	case '[':
		return WBoxL
	case ']':
		return WBoxR
	}
	return Empty
}

func (e *Element) Print() {
	switch *e {
	case Empty:
		fmt.Print(".")
	case Box:
		fmt.Print("O")
	case WBoxL:
		fmt.Print("[")
	case WBoxR:
		fmt.Print("]")
	case Wall:
		fmt.Print("#")
	}
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
	return 0, 0
}

func (d Direction) ToString() string {
	switch d {
	case Up:
		return "Up"
	case Right:
		return "Right"
	case Down:
		return "Down"
	case Left:
		return "Left"
	}
	panic("UNKNOWN DIRECTION")
}

type Robot utils.Vec2

func (r *Robot) Move(direction Direction) {
	dx, dy := direction.ToDiffs()
	r.X += dx
	r.Y += dy
}

type Warehouse struct {
	Robot Robot
	Map   [][]Element
	Moves []Direction
}

func NewWarehouse(input string) Warehouse {
	parts := strings.Split(input, "\n\n")

	// parse map
	lines := strings.Split(parts[0], "\n")

	robot := Robot{}
	parsedMap := make([][]Element, len(lines))

	for y, line := range lines {
		parsedMap[y] = make([]Element, len(line))

		for x, r := range line {
			switch r {
			case '#':
				parsedMap[y][x] = Wall
			case 'O':
				parsedMap[y][x] = Box
			case '@':
				robot.X = x
				robot.Y = y
			default:
				parsedMap[y][x] = Empty
			}
		}
	}

	// parse moves
	allMoves := strings.ReplaceAll(parts[1], "\n", "")
	parsedMoves := parseMoves(allMoves)

	return Warehouse{
		Robot: robot,
		Map:   parsedMap,
		Moves: parsedMoves,
	}
}

func NewWideWarehouse(input string) Warehouse {
	parts := strings.Split(input, "\n\n")

	// parse map
	lines := strings.Split(parts[0], "\n")

	robot := Robot{}
	parsedMap := make([][]Element, len(lines))

	for y, line := range lines {
		parsedMap[y] = make([]Element, len(line)*2)

		for x, r := range line {
			switch r {
			case '#':
				parsedMap[y][x*2] = Wall
				parsedMap[y][x*2+1] = Wall
			case 'O':
				parsedMap[y][x*2] = WBoxL
				parsedMap[y][x*2+1] = WBoxR
			case '@':
				robot.X = x * 2
				robot.Y = y
			default:
				parsedMap[y][x*2] = Empty
				parsedMap[y][x*2+1] = Empty
			}
		}
	}

	// parse moves
	allMoves := strings.ReplaceAll(parts[1], "\n", "")
	parsedMoves := parseMoves(allMoves)

	return Warehouse{
		Robot: robot,
		Map:   parsedMap,
		Moves: parsedMoves,
	}
}

func parseMoves(input string) []Direction {
	parsedMoves := make([]Direction, len(input))
	for i, r := range input {
		switch r {
		case '^':
			parsedMoves[i] = Up
		case '>':
			parsedMoves[i] = Right
		case 'v':
			parsedMoves[i] = Down
		case '<':
			parsedMoves[i] = Left
		}
	}
	return parsedMoves
}

func (w *Warehouse) PrintMap() {
	for y, row := range w.Map {
		for x, e := range row {
			if x == w.Robot.X && y == w.Robot.Y {
				fmt.Print("@")
			} else {
				e.Print()
			}
		}
		fmt.Println()
	}
}

func (w *Warehouse) GetMapSize() (int, int) {
	return len(w.Map), len(w.Map[0])
}

func (w *Warehouse) RunMoves() {
	for _, move := range w.Moves {
		if w.MoveBoxes(move) {
			w.Robot.Move(move)
		}
	}
}

func (w *Warehouse) MoveBoxes(direction Direction) bool {
	dx, dy := direction.ToDiffs()
	x := w.Robot.X + dx
	y := w.Robot.Y + dy

	type ToMove struct {
		X int
		Y int
		E Element
	}

	toCheck := utils.Stack[ToMove]{}
	toCheck.Push(ToMove{X: x, Y: y, E: w.Map[y][x]})
	toMove := []ToMove{}
	checked := map[ToMove]bool{}

	for len(toCheck) > 0 {
		curr := toCheck.Pop()

		if _, ok := checked[*curr]; ok {
			continue
		}

		checked[*curr] = true

		if curr.E == Wall {
			return false
		}

		if curr.E.IsBox() {
			next := ToMove{X: curr.X + dx, Y: curr.Y + dy, E: w.Map[curr.Y+dy][curr.X+dx]}
			toCheck.Push(next)

			if direction == Up || direction == Down {
				if curr.E == WBoxL {
					next := ToMove{X: curr.X + 1, Y: curr.Y, E: w.Map[curr.Y][curr.X+1]}
					toCheck.Push(next)
				} else if curr.E == WBoxR {
					next := ToMove{X: curr.X - 1, Y: curr.Y, E: w.Map[curr.Y][curr.X-1]}
					toCheck.Push(next)
				}
			}

			toMove = append(toMove, *curr)
		}
	}

	slices.Reverse(toMove)
	for _, p := range toMove {
		w.Map[p.Y][p.X] = Empty
		w.Map[p.Y+dy][p.X+dx] = p.E
	}

	return true
}

func (w *Warehouse) SumBoxesGPS() int {
	result := 0
	for y, row := range w.Map {
		for x, e := range row {
			if e == Box || e == WBoxL {
				result += y*100 + x
			}
		}
	}
	return result
}

func main() {
	// input := utils.ReadFile("day15/test.txt")
	input := utils.ReadFile("day15/input.txt")

	warehouseP1 := NewWarehouse(input)
	warehouseP1.RunMoves()

	warehouseP2 := NewWideWarehouse(input)
	warehouseP2.RunMoves()

	fmt.Println()
	fmt.Println("=== day 15 ===")
	fmt.Println("part 1:", warehouseP1.SumBoxesGPS())
	fmt.Println("part 2:", warehouseP2.SumBoxesGPS())
	fmt.Println("==============")
}
