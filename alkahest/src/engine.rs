use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use crate::prelude::*;
use crate::systems::console::ConsoleSystem;

lazy_static! {
    pub static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

/// The top-level context struct for the entire engine.
///
/// This struct contains all of the systems that make up the engine. It is
/// currently passed to the update and cleanup methods so that it can be
/// handled properly. If possible, this struct should be removed and each
/// system can manage itself.
pub(crate) struct EngineContext {
    message_bus: MessageBus,
    console: ConsoleSystem,
}

/// Initialize the engine context
pub(crate) fn init() -> EngineContext {
    // Initialize logger and handle keyboard interrupt
    env_logger::init();

    trace!("Initializing Message Bus...");
    let mut bus = MessageBus::new(Some(1024));
    
    // Register quit message handler
    bus.register(handle_quit_message);
    let s = bus.sender.clone();
    ctrlc::set_handler(move || {
        s.send(Message::Quit).unwrap();
    }).expect("Error setting Ctrl-C handler");

    trace!("Initializing Alkahest systems...");

    let console = ConsoleSystem::init(&mut bus);

    trace!("Alkahest systems initialized.");

    EngineContext {
        message_bus: bus,
        console,
    }
}

/// Update all engine systems
pub(crate) fn update(ctx: &mut EngineContext, timestep: f64) {
    profile! (ProcessMessages, {
        // Process all messages
        ctx.message_bus.process_messages(1024);
    });

    ctx.message_bus.sender.send(Message::Update(timestep)).unwrap();
}

/// Cleanup all engine systems
pub(crate) fn cleanup(ctx: &mut EngineContext) {
    trace!("Cleaning up Alkahest systems...");
    ctx.console.cleanup();
}

/// Check if the engine should close
pub(crate) fn should_close() -> bool {
    !RUNNING.load(Ordering::SeqCst)
}

/// When Message::Quit is sent through the bus, this function will handle
/// setting the RUNNING flag to false so that the engine can shut
/// down gracefully.
fn handle_quit_message(msg: Message) {
    match msg {
        Message::Quit => {
            RUNNING.store(false, Ordering::SeqCst);
        },
        _ => {},
    }
}
