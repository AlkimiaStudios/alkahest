use crate::util::messaging::MessageBus;
use flume::Sender;

pub mod console;
pub mod job_system;

pub trait System {
    fn init(bus: &mut MessageBus) -> Self;
    fn cleanup(&mut self);
}
