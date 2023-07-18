use flume::{bounded, unbounded, Receiver, Sender};

pub enum Message {
    Quit,
    Register(Box<dyn Fn>),
    Ping,
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

pub struct MessageBus {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    targets: Vec<Box<dyn Fn>>,
}

impl MessageBus {
    pub fn new(size: Option<usize>) -> Self {
        let (sender, receiver) = match size {
            Some(size) => bounded(size),
            None => unbounded(),
        };

        Self { sender, receiver, targets: vec![] }
    }

    pub fn register(&mut self, callback: Box<dyn Fn>) {
        self.targets.push(callback);
    }

    pub fn process_messages(&self, limit: usize) {
        while let Ok(msg) = self.receiver.recv() {
            todo!();
        }
    }

    fn handle_message(&mut msg: Message) {
        match msg {
            Message::Register(func) => self.register(func),
            _ => (),
        }
    }
}
