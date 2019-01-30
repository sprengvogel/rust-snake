use crate::BLOCK_SIZE;
use piston_window::Context;
use piston_window::G2d;
use piston_window::rectangle;
use piston_window::types::Color;

pub fn to_gui_coord(game_coord: f64) -> f64 {
    game_coord * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: f64, y: f64, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);

    rectangle(color, [gui_x, gui_y,
            BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_small_square(color: Color, start_x: u16, start_y: u16, length: f64, con: &Context, g: &mut G2d) {
    //Center the square
    let start_x = start_x as f64 + (1.0-length)/2.0;
    let start_y = start_y as f64 + (1.0-length)/2.0;

    let gui_start_x = to_gui_coord(start_x);
    let gui_start_y = to_gui_coord(start_y);

    rectangle(color, [gui_start_x, gui_start_y,
            BLOCK_SIZE * length, BLOCK_SIZE * length], con.transform, g);
}
