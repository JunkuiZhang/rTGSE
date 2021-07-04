use game::Game;
use settings::{WINDOW_TITLE, WINDOW_WIDTH};

mod game;
mod settings;
mod world_grid;

fn main() {
    let mut game = Game::new(WINDOW_WIDTH, WINDOW_TITLE);
    game.run();
}
