use macroquad::prelude::*;
use macroquad::window::Conf;
use std::f32::consts::PI;

const TILESIZE: i32 = 32;

const ROWS: usize = 10;
const COLS: usize = 15;

const WINDOW_HEIGHT: i32 = ROWS as i32 * TILESIZE;
const WINDOW_WIDTH: i32 = COLS as i32 * TILESIZE;

const FOV: f32 = 60.0 * (PI / 180.0);

const RES: i32 = 4;
const NUM_RAYS: i32 = WINDOW_WIDTH / RES;

fn window_conf() -> Conf {
    Conf {
        window_title: "boom".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        init();
        next_frame().await
    }
}

fn init() {
    const GRID: [[i32; COLS]; ROWS] = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    for (i, row) in GRID.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            println!("{}{}", i, j);

            // Coordinates in pixels
            let tile_x: f32 = j as f32 * TILESIZE as f32;
            let tile_y: f32 = i as f32 * TILESIZE as f32;

            if cell == 0 {
                draw_rectangle(
                    tile_x,
                    tile_y,
                    (TILESIZE - 1) as f32,
                    (TILESIZE - 1) as f32,
                    WHITE,
                );
            } else if cell == 1 {
                draw_rectangle(
                    tile_x,
                    tile_y,
                    (TILESIZE - 1) as f32,
                    (TILESIZE - 1) as f32,
                    BLACK,
                );
            }
        }
    }
}
