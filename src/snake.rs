use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;
use crate::game::*;
use crate::drawer::*;
use piston_window::*;
use piston_window::types::Color;
use rand::Rng;

const SN_RED: f32 = 205.0;
const SN_GREEN: f32 = 130.0;
const SN_BLUE: f32 = 79.0;
const SN_ALPHA: f32 = 1.0;
const SNAKE_COLOR: Color = [SN_RED/255.0, SN_GREEN/255.0, SN_BLUE/255.0, SN_ALPHA];

pub struct Snake {
    blocks: Vec<Block>,
    direction: Direction,
    munchie: Block
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Snake {
        Snake {
            blocks: vec!(Block{x:x, y:y},Block{x:x-1, y:y},Block{x:x-2, y:y}),
            direction: Direction::Left,
            munchie: Block{x:7, y:7}
        }
    }
    ///Moves the snake one block and redraws it
    pub fn draw_self(&mut self, context: &Context, g: &mut G2d) {
        self.move_snake();
        for block in &mut self.blocks {
            draw_block(SNAKE_COLOR, block.x as f64, block.y as f64, context, g);
        }
    }

    ///Moves the snake one block while checking for collisions
    fn move_snake(&mut self) {
        let old_head = &self.blocks[0];
        let mut new_head = Block{x:old_head.x, y:old_head.y};
        match self.direction {
            Direction::Right => {
                if new_head.x < WINDOW_WIDTH - 1 {
                    new_head.x += 1;
                } else {
                    new_head.x = 0;
                }
            },
            Direction::Left => {
                if new_head.x > 0 {
                    new_head.x -= 1;
                } else {
                    new_head.x = WINDOW_WIDTH - 1;
                }
            },
            Direction::Up => {
                if new_head.y > 0 {
                    new_head.y -= 1;
                } else {
                    new_head.y = WINDOW_HEIGHT - 1;
                }
            },
            Direction::Down => {
                if new_head.y < WINDOW_HEIGHT - 1 {
                    new_head.y += 1;
                } else {
                    new_head.y = 0
                }
            }
        }
        if new_head == self.munchie {
            //If munchie is eaten, dont pop end to get bigger and generate new munchie
            let munchie_x = rand::thread_rng().gen_range(0, WINDOW_WIDTH-1);
            let munchie_y = rand::thread_rng().gen_range(0, WINDOW_HEIGHT-1);
            self.munchie=Block{x: munchie_x,y: munchie_y};
        } else {
            self.blocks.pop();
        }
        self.blocks.insert(0, new_head);
    }
    pub fn set_direction(&mut self, direction: Direction) {
        if !Game::is_opposite_direction(&self.direction, &direction) {
            self.direction = direction;
        }
    }
    pub fn get_munchie(&self) -> &Block {
        &self.munchie
    }
}
