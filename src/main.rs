mod game;
mod input;
mod render;

use crossterm::terminal::enable_raw_mode;
use std::{thread, time::Duration};

const TICK: Duration = Duration::from_millis(100);

fn main() {
    enable_raw_mode().unwrap();

    let mut game = game::Game::new();

    while game.update() {
        input::handle_input(&mut game);
        render::render(&game);
        thread::sleep(TICK);
    }
}
