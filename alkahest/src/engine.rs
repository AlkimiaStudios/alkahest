use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use crate::prelude::*;
use crate::systems::console::ConsoleSystem;

lazy_static! {
    pub static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

pub(crate) struct EngineContext {
    message_bus: MessageBus,
    console: ConsoleSystem,
}

pub(crate) fn init() -> EngineContext {
    // Initialize logger and handle keyboard interrupt
    env_logger::init();
    let r = RUNNING.clone();

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

pub(crate) fn update(ctx: &mut EngineContext, _timestep: f64) {
    // Process all messages
    ctx.message_bus.process_messages(1024);

    // send ping test
    trace!("Sending ping...");
    ctx.message_bus.sender.send(Message::Ping).unwrap();
}

pub(crate) fn cleanup(ctx: &mut EngineContext) {
    trace!("Cleaning up Alkahest systems...");
    ctx.console.cleanup();
}

pub(crate) fn should_close() -> bool {
    !RUNNING.load(Ordering::SeqCst)
}

fn handle_quit_message(msg: Message) {
    match msg {
        Message::Quit => {
            RUNNING.store(false, Ordering::SeqCst);
        },
        _ => {},
    }
}
