use alkahest::{Application, run};
use alkahest::trace;

struct Game;

impl Application for Game {
    fn init(&mut self) {
        trace!("Game::init()");
    }

    fn update(&mut self, timestep: f64) {
        trace!("Game::update({})", timestep);
    }

    fn cleanup(&mut self) {
        trace!("Game::cleanup()");
    }

    fn should_close(&self) -> bool {
        trace!("Game::should_close()");
        false
    }
}

pub fn main() {
    let mut game = Game;
    run(&mut game);
}
