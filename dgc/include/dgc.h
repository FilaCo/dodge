#ifndef DGC_H
#define DGC_H

#include "dgc/event.h"

typedef struct compiler_t *compiler;

typedef enum { SUCCESS } exitcode;

compiler compiler_new(void);
void compiler_free(compiler);

exitcode compiler_run(compiler, int, const char **);

void compiler_subscribe(compiler, const onevent);
void compiler_dispatch(compiler, const event);

#endif // DGC_H
