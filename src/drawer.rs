use crate::BLOCK_SIZE;
use piston_window::Context;
use piston_window::G2d;
use piston_window::rectangle;
use piston_window::types::Color;

pub fn to_gui_coord(game_coord: u16) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_gui_coord_u32(game_coord: u16) -> u32 {
    to_gui_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: u16, y: u16, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);

    rectangle(color, [gui_x, gui_y,
            BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_rectangle(color: Color, start_x: u16, start_y: u16, width: u16, height: u16, con: &Context, g: &mut G2d) {
    let gui_start_x = to_gui_coord(start_x);
    let gui_start_y = to_gui_coord(start_y);

    rectangle(color, [gui_start_x, gui_start_y,
            BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)], con.transform, g);
}
