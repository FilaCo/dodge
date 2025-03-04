#include "dgc.h"

int main(int argc, char **argv) {
  compiler c = compiler_new();

  exitcode exitCode = compiler_run(c, argc, argv);

  compiler_free(c);

  return exitCode;
}
