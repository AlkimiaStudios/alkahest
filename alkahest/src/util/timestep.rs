use std::time::Instant;

static mut LAST_TIMESTEP: Option<Instant> = None;
static mut CURRENT_TIMESTEP: Option<Instant> = None;

/// A struct for managing time within the engine.
///
/// Currently, all Time methods are static. This allows the usage to be
/// as simple as `Time::now()` or `Time::delta()`. This may change in the
/// future as needs change. `Time::update()` should only be called once
/// per frame, otherwise we will end up with inaccurate timesteps. This
/// is currently performed within the `run` function.
pub struct Time {}

impl Time {
    /// Get the current time.
    pub fn now() -> Instant {
        Instant::now()
    }

    /// Update the current and previous timesteps to
    /// correctly calculate the delta time.
    pub fn update() {
        unsafe {
            LAST_TIMESTEP = CURRENT_TIMESTEP;
            CURRENT_TIMESTEP = Some(Time::now());
        }
    }

    /// Get the delta time between the current and previous timesteps.
    pub fn delta() -> f64 {
        unsafe { 
            match (LAST_TIMESTEP, CURRENT_TIMESTEP) {
                (Some(last), Some(current)) => {
                    let delta = current.duration_since(last);
                    delta.as_secs() as f64 + delta.subsec_nanos() as f64 / 1_000_000_000.0
                },
                _ => 0.0
            }
        }
    }
}

