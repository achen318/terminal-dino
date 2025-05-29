# terminal-dino

The Chrome Dino game in the terminal, written in Rust. Due to budget cuts and my lack of ASCII art skills, the dinosaur became a stick figure instead.

This was created using the `crossterm` library to manipulate the terminal's inputs and outputs and the `rand` library for generating obstacle heights.

The game requires your terminal window to have a minimum height of 10 lines and a width of 20 characters. Otherwise, Rust will panic.

## Installation

1. Install Rust.
2. Run `cargo install terminal-dino`.
3. Run `terminal-dino`.

## Controls

Use `Space`/`w`/`Up` to jump, `p` to pause, and `Esc`/`q` to exit.

## Development

1. Install Rust.
2. Clone the repository.
3. Run `cargo run`.
