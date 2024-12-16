package main

import (
	"fmt"
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
		// for _, move := range w.Moves[:1] {
		w.Move(move)
		// if w.IsValidMove(move) {
		// 	w.Robot.Move(move)
		// 	w.MoveBoxes(move)
		// }
	}
}

func (w *Warehouse) Move(direction Direction) {
	dx, dy := direction.ToDiffs()
	x := w.Robot.X + dx
	y := w.Robot.Y + dy

	if w.Map[y][x] == Wall {
		return
	}

	if w.Map[y][x] == Empty {
		w.Robot.Move(direction)
		return
	}

	if w.MoveBoxesP2(x, y, w.Map[y][x], direction) {
		w.Robot.Move(direction)
		w.Map[w.Robot.Y][w.Robot.X] = Empty
	}
}

func (w *Warehouse) MoveBoxesP2(x int, y int, boxType Element, direction Direction) bool {
	// left-right - keep the same, move by 1
	dx, dy := direction.ToDiffs()
	nx := x + dx
	ny := y + dy

	if boxType == Box {
		for w.Map[ny][nx] == Box {
			nx += dx
			ny += dy
		}

		if w.Map[ny][nx] == Wall {
			return false
		}

		w.Map[ny][nx], w.Map[y][x] = w.Map[y][x], w.Map[ny][nx]
		return true
	}

	if direction == Left || direction == Right {
		for w.Map[ny][nx] != Wall && w.Map[ny][nx] != Empty {
			nx += dx
			ny += dy
		}

		if w.Map[ny][nx] == Wall {
			return false
		}

		for nx != x || ny != y {
			w.Map[ny][nx], w.Map[ny-dy][nx-dx] = w.Map[ny-dy][nx-dx], w.Map[ny][nx]
			nx -= dx
			ny -= dy
		}
	}

	if direction == Up || direction == Down {
		type Point utils.Vec2

		valid := true
		toCheck := []Point{Point{X: x, Y: y}}
		toMove := []Point{}
		for len(toCheck) != 0 {
			tempToCheck := []Point{}

			for _, p := range toCheck {
				if w.Map[p.Y][p.X] != Wall {
					toMove = append(toMove, p)
				}
				nx := p.X + dx
				ny := p.Y + dy

				if w.Map[ny][nx] == Wall {
					valid = false
					break
				}

				if w.Map[ny][nx] != Empty {
					tempToCheck = append(tempToCheck, Point{X: nx, Y: ny})

					if w.Map[ny][nx] == WBoxL {
						tempToCheck = append(tempToCheck, Point{X: nx + 1, Y: ny})
					}
					if w.Map[ny][nx] == WBoxR {
						tempToCheck = append(tempToCheck, Point{X: nx - 1, Y: ny})
					}
				}
			}

			if !valid {
				break
			}

			toCheck = make([]Point, len(tempToCheck))
			copy(toCheck, tempToCheck)
		}

		if valid {
			// set all toMove points to empty
			// move all toMovePoints + direction
		}
	}

	return true

	// // up-down scan for all the boxes that need to be moved
	// if w.Map[y][x] == Wall {
	// 	return false
	// }

	// if w.Map[y][x] == Empty {
	// 	return true
	// }

	// dx, dy := direction.ToDiffs()
	// nx := x + dx
	// ny := y + dy

	// if w.MoveBoxesP2(nx, ny, boxType, direction) {
	// 	if boxType == Box {
	// 		w.Map[ny][nx] = Box
	// 	}

	// 	if boxType == WBoxR && w.Map[ny][nx+1] != WBoxR {
	// 		w.Map[ny][nx] = WBoxL
	// 		w.Map[ny][nx+1] = WBoxR
	// 	}

	// 	if boxType == WBoxL && w.Map[ny][nx-1] != WBoxL {
	// 		w.Map[ny][nx] = WBoxR
	// 		w.Map[ny][nx-1] = WBoxL
	// 	}
	// }

	// return true
}

func (w *Warehouse) MoveBoxes(direction Direction) {
	dx, dy := direction.ToDiffs()
	x := w.Robot.X
	y := w.Robot.Y

	if w.Map[y][x] == Box {
		for w.Map[y][x] == Box {
			x += dx
			y += dy
		}

		w.Map[y][x] = Box
		w.Map[w.Robot.Y][w.Robot.X] = Empty
	}

	if w.Map[y][x] == WBoxL || w.Map[y][x] == WBoxR {
		// push wide box
	}
}

func (w *Warehouse) IsValidMove(move Direction) bool {
	dx, dy := move.ToDiffs()
	nx := w.Robot.X + dx
	ny := w.Robot.Y + dy

	if w.Map[ny][nx] == Wall {
		return false
	}

	for w.Map[ny][nx] == Box || w.Map[ny][nx] == WBoxL || w.Map[ny][nx] == WBoxR {
		nx += dx
		ny += dy
	}

	return w.Map[ny][nx] == Empty
}

func (w *Warehouse) SumBoxesGPS() int {
	result := 0
	for y, row := range w.Map {
		for x, e := range row {
			if e == Box {
				result += y*100 + x
			}
		}
	}
	return result
}

func main() {
	input := utils.ReadFile("day15/test.txt")
	// input := utils.ReadFile("day15/input.txt")

	warehouseP1 := NewWarehouse(input)
	// warehouseP1.PrintMap()
	warehouseP1.RunMoves()
	// warehouseP1.PrintMap()

	warehouseP2 := NewWideWarehouse(input)
	warehouseP2.PrintMap()
	warehouseP2.RunMoves()
	warehouseP2.PrintMap()

	fmt.Println()
	fmt.Println("=== day 15 ===")
	fmt.Println("part 1:", warehouseP1.SumBoxesGPS())
	fmt.Println("part 2:", warehouseP2.SumBoxesGPS())
}
