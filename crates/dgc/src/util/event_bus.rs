#[derive(Debug)]
pub struct EventBus {}

impl EventBus {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dispatch<Event>(&self, ev: Event) {
        todo!()
    }

    pub fn subscribe<CallBack, Event>(&self, cb: CallBack)
    where
        CallBack: FnOnce(Event) + Send + 'static,
    {
        todo!()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
