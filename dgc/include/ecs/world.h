#ifndef ECS_WORLD_H
#define ECS_WORLD_H

typedef void *World;

World w_new(void);
void w_free(World);

#endif // ECS_WORLD_H