pub fn init() {
    env_logger::init();
    trace!("Initializing Alkahest systems...");

    // TODO: Initialize systems
    
    trace!("Alkahest systems initialized.");
}

pub fn update(timestep: f64) {
    trace!("In game loop...")
}

pub fn cleanup() {
    trace!("Cleaning up Alkahest systems...");
}

pub fn should_close() -> bool {
    false
}
