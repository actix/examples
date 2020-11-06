use actix::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Message)]
#[rtype(result = "usize")]
pub struct VisitorCountWrite(pub usize);

#[derive(Message)]
#[rtype(result = "usize")]
pub struct VisitorCountRead();

pub struct StateManager {
    visitor_count: Arc<AtomicUsize>,
}

impl Default for StateManager {
    fn default() -> Self {
        StateManager {
            visitor_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl Actor for StateManager {
    type Context = Context<Self>;
}

impl Handler<VisitorCountWrite> for StateManager {
    type Result = usize;
    fn handle(
        &mut self,
        msg: VisitorCountWrite,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let res = self.visitor_count.fetch_add(msg.0, Ordering::SeqCst);
        res
    }
}

impl Handler<VisitorCountRead> for StateManager {
    type Result = usize;
    fn handle(
        &mut self,
        msg: VisitorCountRead,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        self.visitor_count.load(Ordering::SeqCst)
    }
}
