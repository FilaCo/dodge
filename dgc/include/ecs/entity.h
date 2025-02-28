#ifndef ECS_ENTITY_H
#define ECS_ENTITY_H

#include "inttypes.h"

typedef size_t entity;

typedef struct {
  size_t cap;
  size_t next_pos;
  size_t available;
  entity* buf;
} EntityManager;

EntityManager *em_new(void);
EntityManager *em_with_capacity(const size_t);
void em_free(EntityManager *);

entity em_spawn(EntityManager *);
void em_destroy(EntityManager *, entity);

#endif // ECS_ENTITY_H