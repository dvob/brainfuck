#include "helper.h"
#include <stdio.h>

const size_t MAX_MEMORY = 30000;

typedef enum {
  OP_INC = '+',
  OP_DEC = '-',
  OP_LEFT = '<',
  OP_RIGHT = '>',
  OP_OUTPUT = '.',
  OP_INPUT = ',',
  OP_JUMP_IF_ZERO = '[',
  OP_JUMP_IF_NONZERO = ']',
} Op;

bool run_bf(String *prg) {
  size_t mem[MAX_MEMORY];

  size_t index = 0;

  int inner = 0;

  for (int i = 0; i < prg->len; i++) {
    char c = prg->data[i];
    // fprintf(stderr, "i=%d, c=%c, index=%zu, mem=%zu\n", i, c, index,
    //         mem[index]);

    switch (c) {
    case ' ':
      // fprintf(stderr, "ignore whitespace %d\n", i);
      continue;
      break;
    case '\n':
      // fprintf(stderr, "ignore newline %d\n", i);
      continue;
      break;
    case OP_INC:
      mem[index]++;
      break;
    case OP_DEC:
      mem[index]--;
      break;
    case OP_LEFT:
      if (index == 0) {
        fprintf(stderr, "out of bounds negative on instruction %d\n", i);
        return false;
      }
      index--;
      break;
    case OP_RIGHT:
      index++;
      if (index == MAX_MEMORY) {
        fprintf(stderr, "out of bounds positive on instruction %d\n", i);
        return false;
      }
      break;
    case OP_OUTPUT:
      fputc(mem[index], stdout);
      // fflush(stdout);
      break;
    case OP_INPUT:
      fprintf(stderr, "input not implemented\n");
      return false;
    case OP_JUMP_IF_ZERO:
      inner = 0;
      // do not jump
      if (mem[index] != 0) {
        break;
      }
      // jump if zero
      i++;
      for (;;) {
        if (prg->data[i] == ']' && inner == 0) {
          break;
        }
        if (prg->data[i] == '[') {
          inner++;
        }
        if (prg->data[i] == ']') {
          inner--;
        }
        i++;
      }
      break;
    case OP_JUMP_IF_NONZERO:
      if (mem[index] == 0) {
        break;
      }
      i--;
      for (;;) {
        if (prg->data[i] == '[' && inner == 0) {
          break;
        }
        if (prg->data[i] == ']') {
          inner++;
        }
        if (prg->data[i] == '[') {
          inner--;
        }
        i--;
      }
    default: {
    }
      // fprintf(stderr, "ignore '%c' %d\n", c, i);
    }
  }

  return true;
}

int main(int argc, char **argv) {
  if (argc < 2) {
    fprintf(stderr, "missing argument. usage %s <file>\n", argv[0]);
    return 1;
  }

  String prg = {0};
  if (!read_file(argv[1], &prg)) {
    perror("failed to read file");
  }

  if (!run_bf(&prg)) {
    return 1;
  }
}
