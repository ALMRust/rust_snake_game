use piston_window::{rectangle as _rectangle, Context, G2d, types::Color};
use num_traits::cast::FromPrimitive;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    f64::from(game_coord) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    match to_coord(game_coord) {
        d if d < 0.0 => 0,
        d if d > f64::from(u32::MAX) => u32::MAX,
        d => u32::from_f64(d).expect("Value should have been representable"),
    }
}

pub fn block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    _rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    _rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (f64::from(width)),
            BLOCK_SIZE * (f64::from(height)),
        ],
        con.transform,
        g,
    );
}
