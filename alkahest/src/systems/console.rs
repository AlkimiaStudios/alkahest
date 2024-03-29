use flume::Sender;
use super::System;
use crate::trace;
use crate::util::messaging::{Message, MessageBus};

/// The ConsoleSystem struct.
pub struct ConsoleSystem {
    bus: Sender<Message>,
}

fn handle_message(msg: Message) {
    trace!("{:?}", msg);
}

impl System for ConsoleSystem {
    fn init(bus: &mut MessageBus) -> Self {
        let sender = bus.sender.clone();
        bus.register(handle_message);
        Self { bus: sender }
    }

    fn cleanup(&mut self) {
        drop(self.bus.clone());
    }
}
