#!/usr/bin/env python3

import sys
from enum import Enum

class Inst(Enum):
  right = 1
  left = 2
  inc = 3
  dec = 4
  prt = 5

class Vm:
  def __init__(self):
    self.mem = bytearray(30000)
    self.mp = 0

  def run(self, code):
    for i in code:
      if i == Inst.right:
        self.mp += 1
      elif i == Inst.left:
        self.mp -= 1
      elif i == Inst.inc:
        if self.mem[self.mp] == 255:
          self.mem[self.mp] = 0
        else:
          self.mem[self.mp] = self.mem[self.mp] + 1
      elif i == Inst.dec:
        if self.mem[self.mp] == 0:
          self.mem[self.mp] = 255
        else:
          self.mem[self.mp] = self.mem[self.mp] - 1
      elif isinstance(i, list):
        while self.mem[self.mp] != 0:
          self.run(i)
      elif i == Inst.prt:
        print(chr(self.mem[self.mp]), end='')

def compile(raw_code):
  code = []
  for c in raw_code:
    if c == "<":
      code.append(Inst.left)
    elif c == ">":
      code.append(Inst.right)
    elif c == "+":
      code.append(Inst.inc)
    elif c == "-":
      code.append(Inst.dec)
    elif c == "[":
      code.append(compile(raw_code))
    elif c == "]":
      return code
    elif c == ".":
      code.append(Inst.prt)
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
