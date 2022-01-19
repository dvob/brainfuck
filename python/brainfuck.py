#!/usr/bin/env python3

import sys

RIGHT = 0
LEFT = 1
INC = 2
DEC = 3
LOOP = 4
PRINT = 5

class Vm:
  def __init__(self):
    self.mem = bytearray(30000)
    self.mp = 0

  def run(self, code):
    for i in code:
      if i == RIGHT:
        self.mp += 1
      elif i == LEFT:
        self.mp -= 1
      elif i == INC:
        if self.mem[self.mp] == 255:
          self.mem[self.mp] = 0
        else:
          self.mem[self.mp] = self.mem[self.mp] + 1
      elif i == DEC:
        if self.mem[self.mp] == 0:
          self.mem[self.mp] = 255
        else:
          self.mem[self.mp] = self.mem[self.mp] - 1
      elif isinstance(i, list):
        while self.mem[self.mp] != 0:
          self.run(i)
      elif i == PRINT:
        print(chr(self.mem[self.mp]), end='')

def compile(raw_code):
  code = []
  for c in raw_code:
    if c == "<":
      code.append(LEFT)
    elif c == ">":
      code.append(RIGHT)
    elif c == "+":
      code.append(INC)
    elif c == "-":
      code.append(DEC)
    elif c == "[":
      code.append(compile(raw_code))
    elif c == "]":
      return code
    elif c == ".":
      code.append(PRINT)
  return code

def main():
  if len(sys.argv) < 2:
    print("missing argument: filename")
    exit(1)
  filename = sys.argv[1]
  f = open(filename, "r")
  code = compile(iter(f.read()))
  f.close()

  Vm().run(code)

if __name__ == "__main__":
  main()
