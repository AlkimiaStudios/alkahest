use flume::{bounded, unbounded, Receiver, Sender};

pub type Callback = Box<dyn FnMut(Message)>;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Quit,
    Ping,
    Update(f64),
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

pub struct MessageBus {
    pub sender: Sender<Message>,
    receiver: Receiver<Message>,
    targets: Vec<Callback>,
}

impl MessageBus {
    pub fn new(size: Option<usize>) -> Self {
        let (sender, receiver) = match size {
            Some(size) => bounded(size),
            None => unbounded(),
        };

        Self { sender, receiver, targets: vec![] }
    }

    pub fn register(&mut self, callback: impl FnMut(Message) + 'static) {
        self.targets.push(Box::new(callback));
    }

    pub fn process_messages(&mut self, limit: usize) {
        for msg in self.receiver.try_iter().take(limit) {
            self.targets.iter_mut().for_each(|f| f(msg));
        }
    }
}
