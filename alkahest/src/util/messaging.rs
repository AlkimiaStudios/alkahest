use flume::{bounded, unbounded, Receiver, Sender};

/// The type of message handlers that are registered with the MessageBus.
pub type Callback = Box<dyn FnMut(Message)>;

/// The type of messages that can be sent through the MessageBus.
#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// A message that indicates the engine should quit.
    Quit,
    /// A message that tests access to the MessageBus.
    Ping,
    /// A message that indicates that engine systems should update.
    Update(f64),
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

/// The MessageBus struct.
pub struct MessageBus {
    /// The sender side of the MessageBus.
    ///
    /// This property is public so that it can be cloned by other systems and
    /// they can then send messages through the MessageBus.
    pub sender: Sender<Message>,
    receiver: Receiver<Message>,
    targets: Vec<Callback>,
}

impl MessageBus {
    /// Create a new MessageBus.
    pub fn new(size: Option<usize>) -> Self {
        let (sender, receiver) = match size {
            Some(size) => bounded(size),
            None => unbounded(),
        };

        Self { sender, receiver, targets: vec![] }
    }

    /// Register a callback with the MessageBus.
    pub fn register(&mut self, callback: impl FnMut(Message) + 'static) {
        self.targets.push(Box::new(callback));
    }

    /// Process all messages inside the bus, up to the given limit.
    pub fn process_messages(&mut self, limit: usize) {
        for msg in self.receiver.try_iter().take(limit) {
            self.targets.iter_mut().for_each(|f| f(msg));
        }
    }
}
