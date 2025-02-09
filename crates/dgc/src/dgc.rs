use crate::ecs::{World, WorldId, WorldMgr};
use crate::util::{EventBus, ThreadPool};
use crate::{DGCErr, DGCErrKind, DGCEvent, DGCEventKind, DGCResult};
use std::sync::mpsc;

/// Dodge Compiler (DGC) API.
///
/// In terms of the Dodge language this is a runtime of DGC.
/// It contains a collection of [World] items that are necessary to execute DGC operations in parallel.
pub struct DGC {
    wm: WorldMgr,
    eb: EventBus,
    tp: ThreadPool,
}

impl DGC {
    pub fn new() -> Self {
        todo!()
    }

    pub fn compile(&self) -> DGCResult<()> {
        if self.wm.is_empty() {
            self.wm.spawn("Main".into());
        }

        let (tx, rx) = mpsc::channel();

        self.eb.subscribe(move |ev: DGCEvent| {
            tx.send(ev)
                .expect("unable to send a DGC event to the main thread")
        });

        self.eb
            .dispatch(DGCEvent::new(DGCEventKind::CompilationRequested));

        match rx.recv() {
            Ok(ev) => match ev.kind {
                DGCEventKind::CompilationFailed => {
                    Err(DGCErr::new(DGCErrKind::CompilationFailed, None, None))
                }
                _ => Ok(()),
            },
            Err(err) => Err(err.into()),
        }
    }

    pub fn dispatch(&self, ev: DGCEvent) {
        self.eb.dispatch(ev)
    }

    pub fn subscribe<CallBack>(&self, cb: CallBack)
    where
        CallBack: FnOnce(DGCEvent) + Send + 'static,
    {
        self.eb.subscribe(cb)
    }
}

impl Default for DGC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {}
}
