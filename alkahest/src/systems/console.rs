use crate::{MessageBus, trace};


pub async fn init(bus: MessageBus) {
    while let Ok(msg) = bus.receiver.recv_async().await {
        trace!("{:?}", msg);
    }
}

pub fn cleanup() {
}
