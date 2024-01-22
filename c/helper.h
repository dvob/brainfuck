// Helper function inspierd by nob.h

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  char *data;
  size_t len;
  size_t cap;
} String;

void String_free(String *str) { free(str->data); }

String String_new(const char *value) {
  size_t len = strlen(value);
  char *ptr = (char *)calloc(len, sizeof(*value));

  String str = {
      .data = ptr,
      .len = len,
      .cap = len,
  };
  return str;
}

bool read_file(const char *path, String *str) {
  FILE *f = fopen(path, "rb");
  if (f == NULL) {
    fclose(f);
    return false;
  }

  if (fseek(f, 0, SEEK_END) < 0) {
    fclose(f);
    return false;
  }

  long m = ftell(f);
  if (m < 0) {
    fclose(f);
    return false;
  }

  if (fseek(f, 0, SEEK_SET) < 0) {
    fclose(f);
    return false;
  }

  size_t new_size = str->len + m;
  if (new_size > str->cap) {
    str->data = (char *)realloc(str->data, new_size);
    if (str->data == NULL) {
      fclose(f);
      return false;
    }
  }

  fread(str->data + str->len, m, 1, f);
  if (ferror(f)) {
    fclose(f);
    return false;
  };
  str->len = new_size;

  return true;
}

#define List_append(da, item)                                                  \
  do {                                                                         \
    if ((da)->len >= (da)->cap) {                                              \
      (da)->cap = (da)->cap == 0 ? 1 : (da)->cap * 2;                          \
      (da)->items = realloc((da)->items, (da)->cap * sizeof(*(da)->items));    \
      assert((da)->items != NULL && "out of memory!!!");                       \
    }                                                                          \
                                                                               \
    (da)->items[(da)->len++] = (item);                                         \
  } while (0)

#define List_free(da) free((da).items)

// Append several items to a dynamic array
#define List_append_many(da, new_items, new_items_len)                         \
  do {                                                                         \
    if ((da)->len + new_items_len > (da)->cap) {                               \
      if ((da)->cap == 0) {                                                    \
        (da)->cap = 1;                                                         \
      }                                                                        \
      while ((da)->len + new_items_len > (da)->cap) {                          \
        (da)->cap *= 2;                                                        \
      }                                                                        \
      (da)->items = realloc((da)->items, (da)->cap * sizeof(*(da)->items));    \
      assert((da)->items != NULL && "out of memory!!!");                       \
    }                                                                          \
    memcpy((da)->items + (da)->len, new_items,                                 \
           new_items_len * sizeof(*(da)->items));                              \
    (da)->len += new_items_len;                                                \
  } while (0)
