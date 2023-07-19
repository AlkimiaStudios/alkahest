#![warn(missing_docs)]
//! The simple game engine written in Rust and OpenGL.
//!
//! This is the core library for the Alkahest game engine. This crate is in
//! active development, see the Features section for more information on the
//! current state of the engine.
//!
//! # Features
//! - [X] Game entrypoint
//! - [ ] Window creation

extern crate log;
pub use log::{trace, debug, info, warn, error};

/// Commonly used types and traits.
pub mod prelude;

/// The core engine.
mod engine;
/// The systems that make up the engine.
mod systems;
/// Utility functions.
mod util;

/// The Application trait allows the user to define their own game logic.
///
/// Logic is defined using the methods of the Application trait. These methods
/// function as hooks that are called at different points in the game loop. When
/// creating a game, the user should implement the Application trait for their
/// game struct. The run function will then call the methods of the Application
/// trait at the appropriate times.
pub trait Application {
    /// The init function.
    ///
    /// This function is called once at the beginning of the game loop. This
    /// function should be used to initialize any game state that is needed.
    fn init(&mut self);

    /// The update function.
    ///
    /// This function is called once per frame. This function should be used to
    /// update the game state.
    fn update(&mut self, timestep: f64);

    /// The cleanup function.
    ///
    /// This function is called once at the end of the game loop. This function
    /// should be used to clean up any game state that was initialized in the
    /// init function.
    fn cleanup(&mut self);
    
    /// The should_close function.
    ///
    /// This function is called once per frame. This function should be used to
    /// determine if the game loop should continue running.
    fn should_close(&self) -> bool;
}

/// The run function.
///
/// This function is the entrypoint for the game engine. This function should
/// be called with a mutable reference to a struct that implements the
/// Application trait. The run function will then call the methods of the
/// Application trait at the appropriate times.
pub fn run<T>(app: &mut T) where T: Application {
    use util::timestep::Time;

    // Initialize engine first, then the app
    let mut engine_context = engine::init();
    app.init();

    while !app.should_close() && !engine::should_close() {
        // Grab the delta time since last frame start
        let timestep = Time::delta();

        // Process updates
        engine::update(&mut engine_context, timestep);
        app.update(timestep);

        // Update the time
        Time::update();
    }

    app.cleanup();
    engine::cleanup(&mut engine_context);
}
