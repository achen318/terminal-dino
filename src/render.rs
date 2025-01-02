use crate::game::Game;

use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal::{Clear, ClearType::All}, cursor::{Hide, MoveTo}, style::{Print, PrintStyledContent, Stylize}
};

const DINO_X: u16 = 3;
const WIDTH: u16 = 100;
const HEIGHT: u16 = 10;

pub fn render(game: &Game) {
    let mut stdout = io::stdout();

    stdout.execute(Clear(All)).unwrap();

    // Draw the borders
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if (x == 0 || x == WIDTH - 1) || (y == 0 || y == HEIGHT - 1) {
                stdout.queue(MoveTo(x, y)).unwrap()
                      .queue(PrintStyledContent("█".green())).unwrap();
            }
        }
    }

    // Draw the dino (or person?)
    stdout.queue(MoveTo(DINO_X, HEIGHT - game.dino_y - 4)).unwrap()
          .queue(Print('O')).unwrap()
          .queue(MoveTo(DINO_X - 1, HEIGHT - game.dino_y - 3)).unwrap()
          .queue(Print(r"\|/")).unwrap()
          .queue(MoveTo(DINO_X - 1, HEIGHT - game.dino_y - 2)).unwrap()
          .queue(Print(r"/ \")).unwrap();
    
    // Draw the obstacles
    for x in 0..WIDTH {
        let height: u16 = game.obstacles[x as usize];

        if height > 0 {
            for y in 0..height {
                stdout.queue(MoveTo(x, HEIGHT - y - 2)).unwrap()
                      .queue(Print("▒")).unwrap();
            }
        }
    }

    // Draw the score
    stdout.queue(MoveTo(WIDTH - 15, 1)).unwrap()
          .queue(PrintStyledContent(format!("Score: {}", game.score).cyan())).unwrap();

    // Update
    stdout.queue(MoveTo(WIDTH, HEIGHT)).unwrap()
          .execute(Hide).unwrap()
          .flush().unwrap();
}
