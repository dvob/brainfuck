package main

import (
	"fmt"
	"os"
)

const (
	ADD = iota
	SUB
	RIGHT
	LEFT
	LOOP
	PRINT
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "missing argument")
		os.Exit(1)
	}

	input, err := os.ReadFile(os.Args[1])
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to read %s: %s\n", os.Args[1], err)
		os.Exit(1)
	}

	c := newCompiler(input)

	c.compile()

	// disassemble(c.code, 0)

	v := newVM(c.code)
	v.run(0)
}

type compiler struct {
	index int
	input []byte
	// loops loop
	loopStack []int
	code      [][]int
}

func newCompiler(input []byte) *compiler {
	return &compiler{
		input:     input,
		code:      [][]int{[]int{}},
		loopStack: []int{0},
	}
}

func (c *compiler) currentLoop() int {
	return c.loopStack[len(c.loopStack)-1]
}

func (c *compiler) enterLoop() {
	newLoopIndex := len(c.code)
	c.emit(LOOP, newLoopIndex)
	c.loopStack = append(c.loopStack, newLoopIndex)
	c.code = append(c.code, []int{})
}

func (c *compiler) exitLoop(op ...int) {
	c.loopStack = c.loopStack[:len(c.loopStack)-1]
}

func (c *compiler) emit(op ...int) {
	c.code[c.currentLoop()] = append(c.code[c.currentLoop()], op...)
}

func (c *compiler) compile() error {
	for c.index < len(c.input) {
		switch c.input[c.index] {
		case '+':
			c.emit(ADD)
		case '-':
			c.emit(SUB)
		case '>':
			c.emit(RIGHT)
		case '<':
			c.emit(LEFT)
		case '.':
			c.emit(PRINT)
		case '[':
			c.enterLoop()
		case ']':
			c.exitLoop()
		}
		c.index++
	}
	return nil
}

func disassemble(code [][]int, level int) {
	i := 0
	for i < len(code[level]) {
		switch code[level][i] {
		case ADD:
			fmt.Printf("%d ADD\n", level)
		case SUB:
			fmt.Printf("%d SUB\n", level)
		case RIGHT:
			fmt.Printf("%d RIGHT\n", level)
		case LEFT:
			fmt.Printf("%d LEFT\n", level)
		case PRINT:
			fmt.Printf("%d PRINT\n", level)
		case LOOP:
			fmt.Printf("%d LOOP %d\n", level, code[level][i+1])
			i += 1
			disassemble(code, level+1)
		default:
			fmt.Println(i, "unknown code")
		}
		i += 1
	}
}

type vm struct {
	mem  [30000]uint8
	mp   int
	code [][]int
}

func newVM(code [][]int) *vm {
	return &vm{
		code: code,
	}
}

func (v *vm) run(loop int) error {
	i := 0
	for i < len(v.code[loop]) {
		switch v.code[loop][i] {
		case ADD:
			v.mem[v.mp] += 1
		case SUB:
			v.mem[v.mp] -= 1
		case RIGHT:
			v.mp += 1
		case LEFT:
			v.mp -= 1
		case PRINT:
			fmt.Printf("%c", v.mem[v.mp])
		case LOOP:
			i++
			innerLoop := v.code[loop][i]
			for v.mem[v.mp] != 0 {
				v.run(innerLoop)
			}
		}
		i += 1
	}
	return nil
}
