use crate::game::Game;

use crossterm::{
    cursor::{Hide, MoveTo},
    style::{Print, PrintStyledContent, Stylize},
    terminal::{disable_raw_mode, size, Clear, ClearType::All},
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
    let (width, height) = get_board_size();

    stdout.execute(Clear(All)).unwrap();

    draw_borders(&mut stdout, width, height);
    draw_dino(&mut stdout, height, game.dino_y);

    // Draw the obstacles
    for x in 0..width {
        let obs_height: u16 = game.obstacles[x as usize];

        if obs_height > 0 {
            draw_obstacle(&mut stdout, x, height, obs_height);
        }
    }

    draw_score(game, &mut stdout, width);
    update_buffer(&mut stdout, height);
}

fn get_board_size() -> (u16, u16) {
    let (width, height) = size().unwrap();

    if width < MIN_WIDTH || height < MIN_HEIGHT {
        panic!(
            "Minimum terminal size is {}x{}. You have {}x{}.",
            MIN_WIDTH, MIN_HEIGHT, width, height
        );
    }

    (min(width, MAX_WIDTH), min(height, MIN_HEIGHT))
}

fn draw_borders(stdout: &mut io::Stdout, width: u16, height: u16) {
    for x in 0..width {
        stdout
            .queue(MoveTo(x, 0))
            .unwrap()
            .queue(PrintStyledContent("█".green()))
            .unwrap()
            .queue(MoveTo(x, height - 1))
            .unwrap()
            .queue(PrintStyledContent("█".green()))
            .unwrap();
    }
    for y in 0..height {
        stdout
            .queue(MoveTo(0, y))
            .unwrap()
            .queue(PrintStyledContent("█".green()))
            .unwrap()
            .queue(MoveTo(width - 1, y))
            .unwrap()
            .queue(PrintStyledContent("█".green()))
            .unwrap();
    }
}

fn draw_obstacle(stdout: &mut io::Stdout, x: u16, height: u16, obs_height: u16) {
    for y in 0..obs_height {
        stdout
            .queue(MoveTo(x, height - y - 2))
            .unwrap()
            .queue(Print("▒"))
            .unwrap();
    }
}

fn draw_dino(stdout: &mut io::Stdout, height: u16, dino_y: u16) {
    stdout
        .queue(MoveTo(DINO_X, height - dino_y - 4))
        .unwrap()
        .queue(Print('O'))
        .unwrap()
        .queue(MoveTo(DINO_X - 1, height - dino_y - 3))
        .unwrap()
        .queue(Print(r"\|/"))
        .unwrap()
        .queue(MoveTo(DINO_X - 1, height - dino_y - 2))
        .unwrap()
        .queue(Print(r"/ \"))
        .unwrap();
}

fn draw_score(game: &Game, stdout: &mut io::Stdout, width: u16) {
    stdout
        .queue(MoveTo(width - 15, 1))
        .unwrap()
        .queue(PrintStyledContent(format!("Score: {}", game.score).cyan()))
        .unwrap();
}

fn update_buffer(stdout: &mut io::Stdout, height: u16) {
    stdout
        .queue(MoveTo(0, height))
        .unwrap()
        .execute(Hide)
        .unwrap()
        .flush()
        .unwrap();
}

pub fn reset_cursor() {
    let mut stdout = io::stdout();
    let (_, height) = size().unwrap();

    stdout.queue(MoveTo(0, height - 2)).unwrap();
    disable_raw_mode().unwrap();
}
