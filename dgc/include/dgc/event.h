#ifndef DGC_EVENT_H
#define DGC_EVENT_H

typedef struct event_t *event;

typedef struct eventbus_t *eventbus;

typedef void (*onevent)(const event);

eventbus eventbus_new(void);
void eventbus_free(eventbus);

void eventbus_subscribe(eventbus, const onevent);
void eventbus_dispatch(eventbus, const event);

#endif // DGC_EVENT_H
