#include "ecs/entity.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>

EntityManager *em_new(void) { return em_with_capacity(32); }

EntityManager *em_with_capacity(const size_t cap) {
  EntityManager *pEM;

  pEM = malloc(sizeof(EntityManager) + cap * sizeof(entity));
  if (NULL == pEM) {
    return NULL;
  }

  pEM->cap = cap;
  pEM->next_pos = 0;
  pEM->available = 0;
  pEM->buf = memset(pEM->buf, 0, cap);

  return pEM;
}

void em_free(EntityManager *pEM) {
  if (NULL == pEM) {
    return;
  }

  free(pEM);
}

entity em_spawn(EntityManager *pEM) {
  assert(NULL != pEM);

  if (0 < pEM->available) {
    return recycle_entity(pEM);
  } else {
    return spawn_entity(pEM);
  }
}

void em_destroy(EntityManager *pEM, entity e) {
  assert(NULL != pEM);

  size_t pos = entity_position(e);
  size_t gen = entity_generation(e);

  assert(pos < entity_position_max());
  assert(gen < entity_generation_max());

  pEM->buf[pos] = pEM->next_pos | shift_entity_generation(gen + 1);

  pEM->next_pos = pos;
  ++pEM->available;
}

static entity recycle_entity(EntityManager *pEM) {
  size_t pos = pEM->next_pos;
  entity toBeRecycled = pEM->buf[pos];
  size_t gen = entity_generation(toBeRecycled);

  entity recycled = pos | shift_entity_generation(gen);

  pEM->buf[pos] = recycled;
  pEM->next_pos = entity_position(toBeRecycled);
  --pEM->available;

  return recycled;
}

static entity spawn_entity(EntityManager *pEM) {
  if (pEM->next_pos == pEM->cap) {
    pEM = em_realloc(pEM);

    assert(NULL != pEM);
  }

  entity res = pEM->next_pos;
  pEM->buf[pEM->next_pos] = pEM->next_pos++;

  return res;
}

static EntityManager* em_realloc(EntityManager* pEM) {
  size_t max_pos = entity_position_max();
  assert(pEM->cap < max_pos);

  if (pEM->cap < (max_pos >> 1)) {
    pEM->cap <<= 1;
  } else {
    pEM->cap = max_pos;
  }

  return realloc(pEM, sizeof(EntityManager) + pEM->cap * sizeof(entity));
}

static inline size_t entity_position(entity val) {
  return val & entity_position_mask();
}

static inline size_t entity_generation(entity val) {
  return val >> (sizeof(entity) << 2);
}

static inline size_t shift_entity_generation(size_t val) {
  return val << (sizeof(entity) << 2);
}

static inline const size_t entity_position_mask() {
  switch (sizeof(entity)) {
  case 8:
    return 0xFFFFFFFFFF;
  case 4:
    return 0xFFFFF;
  case 2:
    return 0x3FF;
  }
}

static inline const size_t entity_generation_max() {
  switch (sizeof(entity)) {
  case 8:
    return 0xFFFF;
  case 4:
    return 0xFFF;
  case 2:
    return 0x3F;
  }
}