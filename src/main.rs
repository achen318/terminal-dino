mod game;
mod render;

use std::{thread, time::Duration};

const TICK: Duration = Duration::from_millis(100);

fn main() {
    let mut game = game::Game::new();

    while game.update() {
        render::render(&game);
        thread::sleep(TICK);
    }
}
