use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use crate::job_system;

lazy_static! {
    pub static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

#[derive(Debug)]
pub(crate) struct EngineContext {
    pub jobs: job_system::AsyncContext,
}

pub(crate) fn init() -> EngineContext {
    // Initialize logger and handle keyboard interrupt
    env_logger::init();
    let r = RUNNING.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    trace!("Initializing Alkahest systems...");

    let async_context = job_system::init();
    
    trace!("Alkahest systems initialized.");

    EngineContext{
        jobs: async_context,
    }
}

pub(crate) fn update(_ctx: &mut EngineContext, timestep: f64) {
    trace!("In game loop...")
}

pub(crate) fn cleanup(ctx: &mut EngineContext) {
    trace!("Cleaning up Alkahest systems...");

    job_system::cleanup(&mut ctx.jobs);
}

pub(crate) fn should_close() -> bool {
    !RUNNING.load(Ordering::SeqCst)
}

impl EngineContext {
    pub(crate) fn dispatch_job(&mut self, job: Box<dyn job_system::Job>) {
        self.jobs.dispatch(job);
    }

    pub(crate) fn dispatch_jobs(&mut self, job_count: u32, group_size: u32, inner_job: Box<dyn job_system::Job>) {
        self.jobs.dispatch_many(job_count, group_size, inner_job);
    }
}
