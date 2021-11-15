#include <stdlib.h>
#include <stdio.h>

// Rust からポインタとメモリを開放する関数を受け取り、C内で呼び出して開放
void take_ownership(int *i, void (*dtor)(int *)) {
  printf("got %d\n", *i);
  dtor(i);
}

// C内でメモリをアロケートして参照をRustに返す
int * make_memory() {
  int *i;

  i = malloc(sizeof(int));
  *i = 2;

  return i;
}
