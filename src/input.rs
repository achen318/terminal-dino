use crate::game::Game;

use crossterm::event::{Event::Key, KeyCode, poll, read};
use std::{process::exit, time::Duration};

pub fn handle_input(game: &mut Game) {
    if poll(Duration::from_millis(0)).unwrap() {        
        if let Key(key) = read().unwrap() {
            match key.code {
                KeyCode::Char(' ') => {
                    if !game.jumping {
                        game.jumping = true;
                    }
                },
                KeyCode::Esc => {
                    exit(0);
                },
                _ => {}
            }
        }
    }
}
