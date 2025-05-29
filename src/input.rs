use crate::game::Game;

use crossterm::event::{poll, read, Event::Key, KeyCode};
use std::{process::exit, time::Duration};

pub fn handle_input(game: &mut Game) {
    if poll(Duration::from_millis(0)).unwrap() {
        if let Key(key) = read().unwrap() {
            match key.code {
                KeyCode::Char(' ') | KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                    if !game.jumping {
                        game.jumping = true;
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    exit(0);
                }
                _ => {}
            }
        }
    }
}
