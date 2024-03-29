#ifndef azura_table_h
#define azura_table_h 

#include "common.h"
#include "value.h"

typedef struct Entry{
  ObjString* key;
  Value value;
} Entry;

typedef struct Table {
  int count;
  int capacity;
  Entry* entries;
} Table;

void initTable(Table* table);
void freeTable(Table* table);
bool tableGet(Table* tabel, ObjString* key, Value* value);
bool tableDelete(Table* table, ObjString* key);
bool tableSet(Table* table, ObjString* key, Value value);
void tableAddAll(Table* from, Table* to);
ObjString* tableFindString(Table* table, const char* chars, int length, uint32_t hash);

#endif
