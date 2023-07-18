use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use crate::systems::console;
use crate::MessageBus;

lazy_static! {
    pub static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

#[derive(Debug)]
pub(crate) struct EngineContext {
    message_bus: MessageBus,
}

pub(crate) fn init() -> EngineContext {
    // Initialize logger and handle keyboard interrupt
    env_logger::init();
    let r = RUNNING.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    trace!("Initializing Message Bus...");
    let bus = MessageBus::init();

    trace!("Initializing Alkahest systems...");

    console::init(bus.clone());

    trace!("Alkahest systems initialized.");

    EngineContext {
        message_bus: bus,
    }
}

pub(crate) fn update(ctx: &mut EngineContext, _timestep: f64) {
    ctx.send_message(crate::Message::Ping);
}

pub(crate) fn cleanup(ctx: &mut EngineContext) {
    trace!("Cleaning up Alkahest systems...");
}

pub(crate) fn should_close() -> bool {
    !RUNNING.load(Ordering::SeqCst)
}

impl EngineContext {
    pub fn send_message(&self, msg: crate::Message) {
        self.message_bus.sender.send(msg).map_err(|e| {
            error!("Error sending message: {:?}", e);
        }).ok();
    }
}
