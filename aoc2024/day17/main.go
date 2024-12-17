package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/izoslav/aoc2024/utils"
)

type Opcode int

const (
	ADV Opcode = iota
	BXL
	BST
	JNZ
	BXC
	OUT
	BDV
	CDV
	OpcodeCount
)

func NewOpcode(input int) Opcode {
	if input < 0 || input > int(OpcodeCount) {
		panic("invalid opcode")
	}

	return Opcode(input)
}

func (op Opcode) ToString() string {
	switch op {
	case ADV:
		return "adv"
	case BXL:
		return "bxl"
	case BST:
		return "bst"
	case JNZ:
		return "jnz"
	case BXC:
		return "bxc"
	case OUT:
		return "out"
	case BDV:
		return "bdv"
	case CDV:
		return "CDV"
	default:
		panic("unknown opcode")
	}
}

type Computer struct {
	Data   []int
	Output []int

	A  int // register A
	B  int // register B
	C  int // register C
	PC int // Program Counter

	Ops map[Opcode]func()
}

func NewComputer(filepath string) *Computer {
	input := utils.ReadLines(filepath)

	a := utils.Atoi(input[0][12:])
	b := utils.Atoi(input[1][12:])
	c := utils.Atoi(input[2][12:])

	dataStrs := strings.Split(input[4][9:], ",")
	data := make([]int, len(dataStrs))
	for i, ds := range dataStrs {
		data[i] = utils.Atoi(ds)
	}

	computer := Computer{
		Data:   data,
		Output: []int{},
		A:      a,
		B:      b,
		C:      c,
		PC:     0,
	}

	computer.Ops = map[Opcode]func(){
		ADV: computer.ADV,
		BXL: computer.BXL,
		BST: computer.BST,
		JNZ: computer.JNZ,
		BXC: computer.BXC,
		OUT: computer.OUT,
		BDV: computer.BDV,
		CDV: computer.CDV,
	}

	return &computer
}

func (c *Computer) ReadOutput() string {
	return strings.Trim(strings.Replace(fmt.Sprint(c.Output), " ", ",", -1), "[]")
}

func (c *Computer) ReadData() string {
	return strings.Trim(strings.Replace(fmt.Sprint(c.Data), " ", ",", -1), "[]")
}

func (c *Computer) GetCurrentOP() Opcode {
	return NewOpcode(c.Data[c.PC])
}

func (c *Computer) GetLiteralOperand() int {
	return c.Data[c.PC+1]
}

func (c *Computer) GetComboOperand() int {
	v := c.Data[c.PC+1]

	switch v {
	case 0:
		return v
	case 1:
		return v
	case 2:
		return v
	case 3:
		return v
	case 4:
		return c.A
	case 5:
		return c.B
	case 6:
		return c.C
	case 7:
		// reserver operand
		return 0
	}

	panic("unknown operand")
}

func (c *Computer) IncreasePC(inc int) {
	c.PC = c.PC + inc
}

func (c *Computer) ADV() {
	numerator := c.A
	denominator := int(math.Pow(2, float64(c.GetComboOperand())))

	c.A = numerator / denominator

	c.IncreasePC(2)
}

func (c *Computer) BXL() {
	c.B = c.B ^ c.GetLiteralOperand()

	c.IncreasePC(2)
}

func (c *Computer) BST() {
	c.B = c.GetComboOperand() % 8

	c.IncreasePC(2)
}

func (c *Computer) JNZ() {
	if c.A == 0 {
		c.IncreasePC(2)
		return
	}

	c.PC = c.GetLiteralOperand()
}

func (c *Computer) BXC() {
	c.B = c.B ^ c.C

	// operand read but ignored
	c.IncreasePC(2)
}

func (c *Computer) OUT() {
	c.Output = append(c.Output, c.GetComboOperand()%8)

	c.IncreasePC(2)
}

func (c *Computer) BDV() {
	numerator := c.A
	denominator := int(math.Pow(2, float64(c.GetComboOperand())))

	c.B = numerator / denominator

	c.IncreasePC(2)
}

func (c *Computer) CDV() {
	numerator := c.A
	denominator := int(math.Pow(2, float64(c.GetComboOperand())))

	c.C = numerator / denominator

	c.IncreasePC(2)
}

func (c *Computer) Call(opcode Opcode) {
	c.Ops[opcode]()
}

func (c *Computer) Run() {
	for c.PC < len(c.Data) {
		opcode := c.GetCurrentOP()
		// c.DebugInfo()
		c.Call(opcode)
		// utils.WaitForInput()
	}
}

func (c *Computer) Reset(ra int, rb int, rc int) {
	c.A = ra
	c.B = rb
	c.C = rc
	c.PC = 0
	c.Output = []int{}
}

func FastRun(a int) []int {
	// # a = 66752888
	// # b = 0
	// # c = 0
	// # Program: 2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0
	// # 2 4 - bst 4 - b = a % 8
	// # 1 7 - bxl 7 - b = b ^ 7
	// # 7 5 - cdv 5 - c = a / 2**b
	// # 1 7 - bxl 7 - b = b ^ 7
	// # 0 3 - adv 3 - a = a / 8
	// # 4 1 - bxc 1 - b = b ^ c
	// # 5 5 - out 5 - out b % 8
	// # 3 0 - jnz 0 - jump if a != 0

	result := []int{}
	b := 0
	c := 0

	for a != 0 {
		b = a % 8
		b = b ^ 7
		c = a / (1 << b)
		b = b ^ 7
		a = a >> 3
		b = b ^ c
		result = append(result, b%8)
	}

	return result
}

func main() {
	// filepath := "day17/test.txt"
	filepath := "day17/input.txt"
	c := NewComputer(filepath)
	c.Run()

	firstOutput := c.ReadOutput()

	fmt.Println("=== day 17 ===")
	fmt.Println("part 1a:", firstOutput)
	fmt.Println("part 1b:", FastRun(66752888))
	fmt.Println("part 2 : run day17p2.py")
	fmt.Println("==============")
}
