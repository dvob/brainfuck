#include "helper.h"
#include <stdio.h>
#include <sys/types.h>

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
} OpKind;

typedef struct {
  OpKind kind;
  // change this to u_int32_t reduces the mandelbrot runtime almost by 50%
  size_t operand;
} Op;

typedef struct {
  Op *items;
  size_t len;
  size_t cap;
} Ops;

Op Op_new(OpKind opKind, size_t operand) {
  Op op = {
      .kind = opKind,
      .operand = operand,
  };
  return op;
};

bool bf_read(String *prg, Ops *ops) {

  int index = 0;
  for (int i = 0; i < prg->len; i++) {
    Op op = {0};
    char c = prg->data[i];
    // fprintf(stderr, "i=%d, c=%c, index=%zu, mem=%zu\n", i, c, index,
    //         mem[index]);

    switch (c) {
    case OP_INC:
      List_append(ops, Op_new(OP_INC, 1));
      index++;
      break;
    case OP_DEC:
      List_append(ops, Op_new(OP_DEC, 1));
      index++;
      break;
    case OP_LEFT:
      List_append(ops, Op_new(OP_LEFT, 1));
      index++;
      break;
    case OP_RIGHT:
      List_append(ops, Op_new(OP_RIGHT, 1));
      index++;
      break;
    case OP_OUTPUT:
      List_append(ops, Op_new(OP_OUTPUT, 0));
      index++;
      break;
    case OP_INPUT:
      List_append(ops, Op_new(OP_INPUT, 0));
      index++;
    case OP_JUMP_IF_ZERO:
      List_append(ops, Op_new(OP_JUMP_IF_ZERO, 0));
      index++;
      break;
    case OP_JUMP_IF_NONZERO:
      List_append(ops, Op_new(OP_JUMP_IF_NONZERO, 0));
      index++;
      break;
    }
  }

  for (int i = 0; i < ops->len; i++) {
    if (ops->items[i].kind != OP_JUMP_IF_ZERO) {
      continue;
    };

    int inner = 0;
    for (int j = i + 1; j < ops->len; j++) {
      if (ops->items[j].kind == OP_JUMP_IF_ZERO) {
        inner++;
        continue;
      }
      if (ops->items[j].kind == OP_JUMP_IF_NONZERO) {
        if (inner == 0) {
          ops->items[j].operand = i + 1;
          ops->items[i].operand = j + 1;
          break;
        } else {
          inner--;
        }
      }
    }

    if (inner != 0) {
      fprintf(stderr, "no matching end ] for %d\n", i);
      return false;
    }
  }

  return true;
};

bool bf_run(Ops *ops) {
  size_t mem[30000];
  size_t mem_index = 0;
  Op *items = ops->items;
  for (int i = 0; i < ops->len; i++) {
    // printf("%d %c mem_index=%zu, mem=%zu\n", i, ops->items[i].kind,
    // mem_index, mem[mem_index]);
    switch (items[i].kind) {
    case OP_INC:
      mem[mem_index]++;
      break;
    case OP_DEC:
      mem[mem_index]--;
      break;
    case OP_LEFT:
      mem_index--;
      break;
    case OP_RIGHT:
      mem_index++;
      break;
    case OP_OUTPUT:
      fputc(mem[mem_index], stdout);
      break;
    case OP_INPUT:
      fprintf(stderr, "input not implemented\n");
      return false;
      break;
    case OP_JUMP_IF_ZERO:
      if (mem[mem_index] == 0) {
        i = items[i].operand - 1;
      }
      break;
    case OP_JUMP_IF_NONZERO:
      if (mem[mem_index] != 0) {
        i = items[i].operand - 1;
      }
      break;
    }
  }
  return true;
};

void bf_print(Ops *ops) {
  for (int i = 0; i < ops->len; i++) {
    printf("%d %c(%zu)\n", i, ops->items[i].kind, ops->items[i].operand);
  }
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

  Ops ops = {0};

  if (!bf_read(&prg, &ops)) {
    return 1;
  }

  // bf_print(&ops);

  if (!bf_run(&ops)) {
    return 1;
  }
}
