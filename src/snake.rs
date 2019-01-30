use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;
use crate::game::{Block, Direction};
use crate::drawer::*;
use piston_window::*;
use piston_window::types::Color;

const SN_RED: f32 = 205.0/255.0;
const SN_GREEN: f32 = 130.0/255.0;
const SN_BLUE: f32 = 79.0/255.0;
const SN_ALPHA: f32 = 1.0;
const SNAKE_COLOR: Color = [SN_RED, SN_GREEN, SN_BLUE, SN_ALPHA];

pub struct Snake {
    blocks: Vec<Block>,
    direction: Direction
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Snake {
        Snake {
            blocks: vec!(Block{x:x, y:y},Block{x:x-1, y:y},Block{x:x-2, y:y}),
            direction: Direction::Left
        }
    }
    pub fn draw_self(&mut self, context: &Context, g: &mut G2d) {
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
        self.blocks.pop();
        self.blocks.insert(0, new_head);
        for block in &mut self.blocks {
            draw_block(SNAKE_COLOR, block.x, block.y, context, g);
        }
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
