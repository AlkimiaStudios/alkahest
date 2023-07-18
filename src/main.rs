use alkahest::{Application, run};
use alkahest::trace;

struct Game;

impl Application for Game {
    fn init(&mut self) {
        trace!("Game::init()");
    }

    fn update(&mut self, _timestep: f64) {
    }

    fn cleanup(&mut self) {
        trace!("Game::cleanup()");
    }

    fn should_close(&self) -> bool {
        false
    }
}

pub fn main() {
    let mut game = Game;
    run(&mut game);
}
