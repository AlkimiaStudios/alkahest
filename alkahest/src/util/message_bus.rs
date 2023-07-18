use flume::{bounded, Sender, Receiver};

#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    Ping,
}
unsafe impl Send for Message {}
unsafe impl Sync for Message {}

#[derive(Debug, Clone)]
pub struct MessageBus {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}

unsafe impl Send for MessageBus {}
unsafe impl Sync for MessageBus {}

impl MessageBus {
    pub fn init() -> MessageBus {
        let (sender, receiver) = bounded(256);
        MessageBus {
            sender,
            receiver,
        }
    }
}
