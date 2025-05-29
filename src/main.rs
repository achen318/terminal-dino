mod game;
mod input;
mod render;

use crossterm::terminal::enable_raw_mode;
use render::reset_cursor;
use std::{thread, time::Duration};

const TICK: Duration = Duration::from_millis(100);

fn main() {
    // Initialize game and cursor settings
    enable_raw_mode().unwrap();

    let mut game = game::Game::new();

    // Main game loop
    while game.update() {
        input::handle_input(&mut game);
        render::render(&game);
        thread::sleep(TICK);
    }

    // Reset cursor
    reset_cursor();
}
