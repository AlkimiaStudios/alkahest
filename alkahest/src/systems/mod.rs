use crate::util::messaging::MessageBus;

/// The console system module.
pub mod console;
/// The job system module.
// pub mod job_system;

/// A trait for all systems to implement that defines the interface
/// for initializing and cleaning up after the system.
pub trait System {
    /// Initialize the system.
    fn init(bus: &mut MessageBus) -> Self;
    /// Cleanup the system.
    fn cleanup(&mut self);
}
