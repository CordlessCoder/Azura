#ifndef azura_chunk_h
#define azura_chunk_h

#include "common.h"
#include "value.h"

typedef enum {
  OP_CONSTANT,
  OP_NIL,
  OP_FALSE,
  OP_POP,
  OP_EQUAL,
  OP_GREATER,
  OP_LESS,
  OP_TRUE,
  OP_ADD,
  OP_SUBTRACT,
  OP_MULTIPLY,
  OP_DIVIDE,
  OP_NOT,
  OP_NEGATE,
  OP_INFO,
  OP_GET_LOCAL,
  OP_SET_LOCAL,
  OP_SET_GLOBAL,
  OP_GET_GLOBAL,
  OP_DEFINE_GLOBAL,
  OP_RETRUN,
} OpCode;

typedef struct {
  int count;
  int capacity;
  uint8_t *code;
  int *line;
  ValueArray constants;
} Chunk;

void initChunk(Chunk *chunk);
void freeChunk(Chunk *chunk);
void writeChunk(Chunk *chunk, uint8_t byte, int line);
int addConstants(Chunk *chunk, Value value);

#endif
