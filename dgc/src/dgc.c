#include "dgc.h"
#include "ecs.h"
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EMPTY_FLAGS 0

struct compiler_t {
  eventbus eb;
  worldmanager wm;
};

typedef struct {
} compileargs;

static compileargs parse_args(int, const char **);

compiler compiler_new(void) {
  compiler c = malloc(sizeof(struct compiler_t));

  c->eb = eventbus_new();
  c->wm = worldmanager_new();

  return c;
}
void compiler_free(compiler c) {
  if (NULL == c) {
    return;
  }

  eventbus_free(c->eb);
  worldmanager_free(c->wm);

  free(c);
  c = NULL;
}

exitcode compiler_run(compiler c, int argc, const char **argv) {
  assert(NULL != c);

  compileargs args = parse_args(argc, argv);

  return SUCCESS;
}

void compiler_subscribe(compiler c, const onevent cb) {
  assert(NULL != c);

  eventbus_subscribe(c->eb, cb);
}
void compiler_dispatch(compiler c, const event e) {
  assert(NULL != c);

  eventbus_dispatch(c->eb, e);
}
