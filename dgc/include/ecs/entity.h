#ifndef ECS_ENTITY_H
#define ECS_ENTITY_H

typedef struct entity_t *entity;

typedef struct entitymanager_t *entitymanager;

entitymanager entitymanager_new(void);
void entitymanager_free(entitymanager);

entity entitymanager_spawn(entitymanager);
void entitymanager_destroy(entitymanager, entity);

#endif // ECS_ENTITY_H