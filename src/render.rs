use crate::game::Game;

use crossterm::{
    cursor::{Hide, MoveTo},
    style::{Print, PrintStyledContent, Stylize},
    terminal::{size, Clear, ClearType::All},
    ExecutableCommand, QueueableCommand,
};
use std::cmp::min;
use std::io::{self, Write};

const DINO_X: u16 = 3;
const MIN_WIDTH: u16 = 20;
const MIN_HEIGHT: u16 = 10;
const MAX_WIDTH: u16 = 100;

pub fn render(game: &Game) {
    let mut stdout = io::stdout();
    let (mut width, mut height) = size().unwrap();

    if width < MIN_WIDTH || height < MIN_HEIGHT {
        panic!(
            "Minimum terminal size is {}x{}. You have {}x{}.",
            MIN_WIDTH, MIN_HEIGHT, width, height
        );
    }

    width = min(width, MAX_WIDTH);
    height = min(height, MIN_HEIGHT);

    stdout.execute(Clear(All)).unwrap();

    // Draw the borders
    for x in 0..width {
        for y in 0..height {
            if (x == 0 || x == width - 1) || (y == 0 || y == height - 1) {
                stdout
                    .queue(MoveTo(x, y))
                    .unwrap()
                    .queue(PrintStyledContent("█".green()))
                    .unwrap();
            }
        }
    }

    // Draw the dino (person)
    stdout
        .queue(MoveTo(DINO_X, height - game.dino_y - 4))
        .unwrap()
        .queue(Print('O'))
        .unwrap()
        .queue(MoveTo(DINO_X - 1, height - game.dino_y - 3))
        .unwrap()
        .queue(Print(r"\|/"))
        .unwrap()
        .queue(MoveTo(DINO_X - 1, height - game.dino_y - 2))
        .unwrap()
        .queue(Print(r"/ \"))
        .unwrap();

    // Draw the obstacles
    for x in 0..width {
        let obs_height: u16 = game.obstacles[x as usize];

        if obs_height > 0 {
            for y in 0..obs_height {
                stdout
                    .queue(MoveTo(x, height - y - 2))
                    .unwrap()
                    .queue(Print("▒"))
                    .unwrap();
            }
        }
    }

    // Draw the score
    stdout
        .queue(MoveTo(width - 15, 1))
        .unwrap()
        .queue(PrintStyledContent(format!("Score: {}", game.score).cyan()))
        .unwrap();

    // Update
    stdout
        .queue(MoveTo(width, height - 1))
        .unwrap()
        .execute(Hide)
        .unwrap()
        .flush()
        .unwrap();
}
