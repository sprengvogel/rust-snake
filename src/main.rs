extern crate piston_window;
extern crate rand;

mod game;
mod snake;
mod drawer;

use piston_window::*;
use piston_window::types::Color;
use crate::game::Game;

const BG_RED: f32 = 35.0;
const BG_GREEN: f32 = 130.0;
const BG_BLUE: f32 = 79.0;
const BG_ALPHA: f32 = 1.0;
const BACKGROUND_COLOR: Color = [BG_RED/255.0, BG_GREEN/255.0, BG_BLUE/255.0, BG_ALPHA];

const WINDOW_WIDTH: u16 = 20;
const WINDOW_HEIGHT: u16 = 20;

///Size of a block in px, blocks are used as coordinates
const BLOCK_SIZE: f64 = 25.0;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", (WINDOW_HEIGHT as f64 * BLOCK_SIZE, WINDOW_WIDTH as f64 * BLOCK_SIZE))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut game = Game::new();

    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |_context, g| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&_context,g);
        });
    }
}
