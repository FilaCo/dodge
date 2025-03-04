#include "dgc.h"
#include <stdlib.h>

struct compiler_t {
  eventbus eb;
};

compiler compiler_new(void) {
  compiler c = malloc(sizeof(struct compiler_t));

  c->eb = eventbus_new();

  return c;
}
void compiler_free(compiler c) {
  if (NULL == c) {
    return;
  }

  eventbus_free(c->eb);

  free(c);
}

exitcode compiler_run(compiler c, int arc, const char **argv) {
  return SUCCESS;
}

void compiler_subscribe(compiler c, const onevent cb) {
  eventbus_subscribe(c->eb, cb);
}
void compiler_dispatch(compiler c, const event e) { eventbus_dispatch(c->eb, e); }
