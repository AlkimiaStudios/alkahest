use std::time::Instant;

static mut LAST_TIMESTEP: Option<Instant> = None;
static mut CURRENT_TIMESTEP: Option<Instant> = None;

pub struct Time {}

impl Time {
    pub fn now() -> Instant {
        Instant::now()
    }

    pub fn update() {
        unsafe {
            LAST_TIMESTEP = CURRENT_TIMESTEP;
            CURRENT_TIMESTEP = Some(Time::now());
        }
    }

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

