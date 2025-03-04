#ifndef ECS_ENTITY_H
#define ECS_ENTITY_H

#include "inttypes.h"

typedef size_t entity;

typedef struct entitymanager_t *entitymanager;

entitymanager entitymanager_new(void);
entitymanager entitymanager_with_capacity(size_t);
void entitymanager_free(entitymanager);

entity entitymanager_spawn(entitymanager);
void entitymanager_destroy(entitymanager, entity);

#endif // ECS_ENTITY_H