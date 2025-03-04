#ifndef UTIL_EVENTBUS_H
#define UTIL_EVENTBUS_H

typedef struct event_t *event;

typedef struct eventbus_t *eventbus;

typedef void (*onevent)(const event);

eventbus eventbus_new(void);
void eventbus_free(eventbus);

void eventbus_subscribe(eventbus, const onevent);
void eventbus_dispatch(eventbus, const event);

#endif // UTIL_EVENTBUS_H
