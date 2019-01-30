use piston_window::*;
use piston_window::types::Color;
use crate::snake::Snake;
pub struct Game {
    snake: Snake,
}
impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(3,3)
        }
    }

    pub fn draw(&mut self, context: &Context, g: &mut G2d) {
        self.snake.draw_self(context, g);
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Right|Key::D => self.snake.set_direction(Direction::Right),
            Key::Left|Key::A => self.snake.set_direction(Direction::Left),
            Key::Up|Key::W => self.snake.set_direction(Direction::Up),
            Key::Down|Key::S => self.snake.set_direction(Direction::Down),
            _ => (),
        }
    }
}

pub enum Direction {
    Left,
    Right,
    Down,
    Up
}

pub struct Block {
    pub x: u16,
    pub y: u16
}
