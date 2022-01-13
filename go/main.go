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
	JUMP
	CONDJUMP
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

	//disassemble(c.code)

	v := newVM(c.code)
	v.run()
	//fmt.Println(inst)
}

type compiler struct {
	input []byte
	addrs []int
	code  []int
}

func newCompiler(input []byte) *compiler {
	return &compiler{
		input: input,
	}
}

func (c *compiler) emit(op ...int) {
	c.code = append(c.code, op...)
}

func (c *compiler) compile() int {
	i := 0
	for i < len(c.input) {
		switch c.input[i] {
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
			c.addrs = append(c.addrs, len(c.code))
			c.emit(CONDJUMP, -1)
		case ']':
			addr := c.addrs[len(c.addrs)-1]
			c.addrs = c.addrs[:len(c.addrs)-1]
			c.emit(JUMP, addr)
			c.code[addr+1] = len(c.code)
		}
		i += 1
	}
	return i

}

func disassemble(code []int) {
	i := 0
	for i < len(code) {
		switch code[i] {
		case ADD:
			fmt.Printf("%d ADD\n", i)
		case SUB:
			fmt.Printf("%d SUB\n", i)
		case RIGHT:
			fmt.Printf("%d RIGHT\n", i)
		case LEFT:
			fmt.Printf("%d LEFT\n", i)
		case PRINT:
			fmt.Printf("%d PRINT\n", i)
		case JUMP:
			fmt.Printf("%d JUMP %d\n", i, code[i+1])
			i += 1
		case CONDJUMP:
			fmt.Printf("%d CONDJUMP %d\n", i, code[i+1])
			i += 1
		default:
			fmt.Println(i, "unknown code")
		}
		i += 1
	}
}

type vm struct {
	mem  [30000]uint8
	mp   int
	ip   int
	code []int
}

func newVM(code []int) *vm {
	return &vm{
		code: code,
	}
}

func (v *vm) run() error {
	for v.ip < len(v.code) {
		// fmt.Printf("ip=%d, mp=%d, op=%d, mem=%d\n", v.ip, v.mp, v.code[v.ip], v.mem[v.mp])
		switch v.code[v.ip] {
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
		case JUMP:
			v.ip += 1
			addr := v.code[v.ip]
			v.ip = addr
			continue
		case CONDJUMP:
			v.ip += 1
			addr := v.code[v.ip]
			if v.mem[v.mp] == 0 {
				v.ip = addr
				continue
			}
		}
		v.ip += 1
	}
	return nil
}
