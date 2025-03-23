from collections.inline_array import InlineArray
import sys

var RIGHT: UInt16 = 1
var LEFT: UInt16 = 2
var INC: UInt16 = 3
var DEC: UInt16 = 4
var OUT: UInt16 = 5
var JUMP_FWD: UInt16 = 6
var JUMP_BACK: UInt16 = 7

def main():
  if len(sys.argv()) < 2:
    print("missing argument: filename")
    sys.exit(1)
  filename = sys.argv()[1]
  f = open(filename, "r")
  var code = f.read()
  f.close()

  ops = compile(code)

  run(ops)


def compile(data: String) -> List[UInt16]:
    code = List[UInt16]()
    backRefs = List[Int]()
    for c in data.codepoint_slices():
        if c == '>':
            code.append(LEFT)
        elif c == '<':
            code.append(RIGHT)
        elif c == '+':
            code.append(INC)
        elif c == '-':
            code.append(DEC)
        elif c == '.':
            code.append(OUT)
        elif c == '[':
            code.append(JUMP_FWD)
            code.append(0)
            backRefs.append(len(code) - 1)
        elif c == ']':
            code.append(JUMP_BACK)
            if len(backRefs) == 0:
                raise Error("Unmatched ']'")
            backRef = backRefs.pop()
            code.append(backRef+1)
            code[backRef] = len(code)
    return code


def run(code: List[UInt16]):
    var mem = InlineArray[Byte, 30000](fill=0)
    var mp = 0
    var pc = 0
    while pc < len(code):
        var op = code[pc]
        #print("pc", pc, "op", op_to_str(op), "mp", mp, "mem[mp]", mem[mp])
        if op == LEFT:
            mp += 1
        elif op == RIGHT:
            mp -= 1
        elif op == INC:
            mem[mp] = mem[mp] + 1
        elif op == DEC:
            mem[mp] = mem[mp] - 1
        elif op == OUT:
            print(String(chr(Int(mem[mp]))), end="")
        elif op == JUMP_FWD:
            if mem[mp] == 0:
                pc = Int(code[pc+1])
                continue
            else:
                pc += 1
        elif op == JUMP_BACK:
            if mem[mp] != 0:
                pc = Int(code[pc+1])
                continue
            else:
                pc += 1
        pc += 1

def op_to_str(op: UInt16) -> String:
    if op == LEFT:
        return "LEFT"
    elif op == RIGHT:
        return "RIGHT"
    elif op == INC:
        return "INC"
    elif op == DEC:
        return "DEC"
    elif op == OUT:
        return "OUT"
    elif op == JUMP_FWD:
        return "JUMP_FWD"
    elif op == JUMP_BACK:
        return "JUMP_BACK"
    else:
        return "UNKNOWN(" + String(op) + ")"

def show_code(code: List[UInt16]):
    var pc = 0
    while pc < len(code):
        var op = code[pc]
        print(pc, op_to_str(op))
        if op == LEFT:
            print(pc, "LEFT")
        elif op == RIGHT:
            print(pc, "RIGHT")
        elif op == INC:
            print(pc, "INC")
        elif op == DEC:
            print(pc, "DEC")
        elif op == OUT:
            print(pc, "OUT")
        elif op == JUMP_FWD:
            print(pc, "JUMP_FWD")
            pc += 1
            print(pc, code[pc])
        elif op == JUMP_BACK:
            print(pc, "JUMP_BACK")
            pc += 1
            print(pc, code[pc])
        else:
            print("UNKNOWN")
        pc += 1
    print("")
