use piston_window::*;
use piston_window::types::Color;
use crate::snake::Snake;
use crate::drawer::*;
use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;
use std::{thread, time};
use rand::Rng;

const MN_RED: f32 = 50.0;
const MN_GREEN: f32 = 130.0;
const MN_BLUE: f32 = 205.0;
const MN_ALPHA: f32 = 1.0;
const MUNCHIE_COLOR: Color = [MN_RED/255.0, MN_GREEN/255.0, MN_BLUE/255.0, MN_ALPHA];

const GO_RED: f32 = 255.0;
const GO_GREEN: f32 = 255.0;
const GO_BLUE: f32 = 255.0;
const GO_ALPHA: f32 = 1.0;
const GAME_OVER_COLOR: Color = [GO_RED/255.0, GO_GREEN/255.0, GO_BLUE/255.0, GO_ALPHA];

///Length and width of a munchie in blocks
const MUNCHIE_WIDTH: f64 = 0.5;

pub struct Game {
    snake: Snake,
    munchie: Block,
    points: u64,
    games: u64,
    ///Delay between every snake move in ms
    move_delay: u64
}
impl Game {
    pub fn new(move_delay: u64) -> Game {
        Game {
            snake: Snake::new(3,3),
            munchie: Block{x:7, y:7},
            points: 0,
            games: 1,
            move_delay: move_delay,
        }
    }

    pub fn draw(&mut self, context: &Context, g: &mut G2d) {
        thread::sleep(time::Duration::from_millis(self.move_delay));
        let (game_over, munchie_eaten) = self.snake.move_snake(&self.munchie);
        if game_over {
            self.game_over(context, g);
        } else if munchie_eaten {
            self.points+=10;
            self.generate_new_munchie();
        }
        self.snake.draw_self(context, g);
        self.draw_munchie(context, g);
    }

    fn draw_munchie(&self, context: &Context, g: &mut G2d) {
        draw_small_square(MUNCHIE_COLOR, self.munchie.x, self.munchie.y, MUNCHIE_WIDTH, context, g);
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
    pub fn is_opposite_direction(dir1: &Direction, dir2: &Direction) -> bool {
        (*dir1==Direction::Left && *dir2==Direction::Right)||(*dir1==Direction::Right && *dir2==Direction::Left)||(*dir1==Direction::Up && *dir2==Direction::Down)||(*dir1==Direction::Down && *dir2==Direction::Up)
    }

    fn generate_new_munchie(&mut self) {
        let mut munchie_x = rand::thread_rng().gen_range(0, WINDOW_WIDTH-1);
        let mut munchie_y = rand::thread_rng().gen_range(0, WINDOW_HEIGHT-1);
        let mut new_munchie = Block{x: munchie_x,y: munchie_y};
        while self.snake.check_self_collision(&new_munchie) {
            munchie_x = rand::thread_rng().gen_range(0, WINDOW_WIDTH-1);
            munchie_y = rand::thread_rng().gen_range(0, WINDOW_HEIGHT-1);
            new_munchie = Block{x: munchie_x,y: munchie_y};
        }
        self.munchie=new_munchie;
    }

    fn game_over(&mut self, context: &Context, g: &mut G2d) {
        draw_rectangle(GAME_OVER_COLOR, 0, 0, WINDOW_HEIGHT as f64, WINDOW_WIDTH as f64, context, g);
        println!("Game over! You scored {} points in game {}.", self.points, self.games);
        thread::sleep(time::Duration::from_millis(1000));
        self.games+=1;
        self.snake = Snake::new(3,3);
        self.munchie = Block{x:7, y:7};
        self.points = 0;
    }
}

#[derive(PartialEq,Eq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up
}

#[derive(PartialEq,Eq)]
pub struct Block {
    pub x: u16,
    pub y: u16
}
