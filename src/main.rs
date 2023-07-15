use alkahest::{Application, run};

struct Game;

impl Application for Game {
    fn init(&mut self) {
        println!("Game::init()");
    }

    fn update(&mut self, timestep: f64) {
        println!("Game::update({})", timestep);
    }

    fn cleanup(&mut self) {
        println!("Game::cleanup()");
    }

    fn should_close(&self) -> bool {
        println!("Game::should_close()");
        false
    }
}

pub fn main() {
    let mut game = Game;
    run(&mut game);
}
