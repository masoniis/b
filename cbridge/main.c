#include <stdio.h>

#include "bcraft.h"

int main() {
  printf("Calling Rust function from C...\n");
  int32_t result = run_the_project();
  printf("Rust function returned: %d\n", result);
  return result;
}
