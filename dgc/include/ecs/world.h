#ifndef ECS_WORLD_H
#define ECS_WORLD_H

typedef struct world_t *world;

typedef struct worldmanager_t *worldmanager;

worldmanager worldmanager_new(void);
void worldmanager_free(worldmanager);

#endif // ECS_WORLD_H