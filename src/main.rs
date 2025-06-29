use macroquad::prelude::*;
use macroquad::window::Conf;
use std::f32::consts::PI;

const TILESIZE: i32 = 96;

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
    let mut x: f32 = WINDOW_WIDTH as f32 / 2.0;
    let mut y: f32 = WINDOW_HEIGHT as f32 / 2.0;
    let mut rotation_angle: f32 = 0.0;
    loop {
        clear_background(BLACK);
        init_grid();
        init_player(&mut x, &mut y, &mut rotation_angle);
        draw_rays(x, y, rotation_angle);
        next_frame().await
    }
}

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

fn init_grid() {
    for (i, row) in GRID.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
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

fn init_player(x: &mut f32, y: &mut f32, rotation_angle: &mut f32) {
    let radius: f32 = 4.0;

    // Using multiple if statements instead of a single if-else if statement to ensure that multiple key presses are handled correctly
    if is_key_down(KeyCode::Up) {
        let next_x = *x + (1.0 * rotation_angle.cos());
        let next_y = *y + (1.0 * rotation_angle.sin());
        if !has_wall_at(next_x, next_y) {
            *x = next_x;
            *y = next_y;
        }
    }
    if is_key_down(KeyCode::Down) {
        let next_x = *x - (1.0 * rotation_angle.cos());
        let next_y = *y - (1.0 * rotation_angle.sin());
        if !has_wall_at(next_x, next_y) {
            *x = next_x;
            *y = next_y;
        }
    }
    if is_key_down(KeyCode::Right) {
        *rotation_angle += 0.5 * (PI / 180.0);
    }
    if is_key_down(KeyCode::Left) {
        *rotation_angle -= 0.5 * (PI / 180.0);
    }

    draw_circle(*x, *y, radius, RED);
}

fn draw_rays(x: f32, y: f32, rotation_angle: f32) {
    let start_angle: f32 = rotation_angle - (FOV / 2.0);
    let end_angle: f32 = rotation_angle + (FOV / 2.0);
    let mut ray_angle: f32 = start_angle;

    let gap_angle: f32 = FOV / (NUM_RAYS - 1) as f32;

    while ray_angle <= end_angle {
        let mut x2 = x + ray_angle.cos() * 1e-4;
        let mut y2 = y + ray_angle.sin() * 1e-4;
        loop {
            let x1_snap = snap_x(x2, ray_angle);
            let y2_snap = snap_y(y2, ray_angle);

            let y1_snap = (ray_angle.tan() * (x1_snap - x2)) + y2;
            let x2_snap = ((y2_snap - y2) / ray_angle.tan()) + x2;

            // calculate the distance of x_snap to ray distance
            let len_snap_1 = distance(x1_snap, y1_snap, x2, y2);
            let len_snap_2 = distance(x2_snap, y2_snap, x2, y2);

            if len_snap_1 >= len_snap_2 {
                // draw_circle(x2_snap, y2_snap, 20.0, GREEN);
                draw_line(x2, y2, x2_snap, y2_snap, 2.0, RED);
                if has_wall_at(x2_snap, y2_snap) {
                    break;
                }
                x2 = x2_snap;
                y2 = y2_snap;
            }
            if len_snap_1 < len_snap_2 {
                // draw_circle(x1_snap, y1_snap, 20.0, GREEN);
                draw_line(x2, y2, x1_snap, y1_snap, 2.0, RED);
                if has_wall_at(x1_snap, y1_snap) {
                    break;
                }
                x2 = x1_snap;
                y2 = y1_snap;
            }
        }
        ray_angle += gap_angle;
    }
}

fn snap_x(pixel_coordinate: f32, rotation_angle: f32) -> f32 {
    if rotation_angle.cos() >= 0.0 {
        (pixel_coordinate / TILESIZE as f32).ceil() * TILESIZE as f32 + 1e-4
    } else if rotation_angle.cos() < 0.0 {
        (pixel_coordinate / TILESIZE as f32).floor() * TILESIZE as f32 - 1e-4
    } else {
        unreachable!()
    }
}
fn snap_y(pixel_coordinate: f32, rotation_angle: f32) -> f32 {
    if rotation_angle.sin() > 0.0 {
        (pixel_coordinate / TILESIZE as f32).ceil() * TILESIZE as f32 + 1e-4
    } else if rotation_angle.sin() <= 0.0 {
        (pixel_coordinate / TILESIZE as f32).floor() * TILESIZE as f32 - 1e-4
    } else {
        unreachable!()
    }
}
fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // distance formula
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
fn has_wall_at(x_pixel_coordinate: f32, y_pixel_coordinate: f32) -> bool {
    if x_pixel_coordinate < 0.0
        || x_pixel_coordinate >= WINDOW_WIDTH as f32
        || y_pixel_coordinate < 0.0
        || y_pixel_coordinate >= WINDOW_HEIGHT as f32
    {
        return true;
    }
    GRID[(y_pixel_coordinate / TILESIZE as f32).floor() as usize]
        [(x_pixel_coordinate / TILESIZE as f32).floor() as usize]
        == 1
}
