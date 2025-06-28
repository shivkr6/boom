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

fn init_player(x: &mut f32, y: &mut f32, rotation_angle: &mut f32) {
    let radius: f32 = 4.0;

    // Using multiple if statements instead of a single if-else if statement to ensure that multiple key presses are handled correctly
    if is_key_down(KeyCode::Up) {
        *x += 1.0 * rotation_angle.cos();
        *y += 1.0 * rotation_angle.sin();
    }
    if is_key_down(KeyCode::Down) {
        *x -= 1.0 * rotation_angle.cos();
        *y -= 1.0 * rotation_angle.sin();
    }
    if is_key_down(KeyCode::Right) {
        *rotation_angle += 0.1 * (PI / 180.0);
    }
    if is_key_down(KeyCode::Left) {
        *rotation_angle -= 0.1 * (PI / 180.0);
    }

    draw_circle(*x, *y, radius, RED);
}

fn draw_rays(x: f32, y: f32, rotation_angle: f32) {
    let start_angle: f32 = rotation_angle - (FOV / 2.0);
    let end_angle: f32 = rotation_angle + (FOV / 2.0);
    let mut temp_angle: f32 = start_angle;

    let gap_angle: f32 = FOV / (NUM_RAYS - 1) as f32;

    // while temp_angle <= end_angle {
    //     // ending ray position assume karege kuch
    //     // fir check karege step loop mai (use DDA simple wala) ki wall se hit ho rhi kya ray by converting coordinates to grid
    //     // if it hits the grid then we stop and store the distance into a variable
    //     // fir ye draw line wala function execute kar dege aur vo ray ko draw kar dege
    //     draw_line(
    //         x,
    //         y,
    //         x + temp_angle.cos() * 500.0,
    //         y + temp_angle.sin() * 500.0,
    //         2.0,
    //         RED,
    //     );
    //     temp_angle += gap_angle;
    // }

    let x2 = x + rotation_angle.cos() * 500.0;
    let y2 = y + rotation_angle.sin() * 500.0;

    let x1_snap = snap_x(x2, rotation_angle);
    let y2_snap = snap_y(y2, rotation_angle);

    let y1_snap = (rotation_angle.tan() * (x1_snap - x)) + y;
    let x2_snap = ((y2_snap - y) / rotation_angle.tan()) + x;

    // calculate the distance of x_snap to ray distance without snapping

    let snap_delta_x = distance(x1_snap, y2, x2, y2);
    let snap_delta_y = distance(x2, y2_snap, x2, y2);

    draw_circle(x2_snap, y2_snap, 20.0, GREEN);
    draw_circle(x1_snap, y1_snap, 20.0, YELLOW);
    // if snap_delta_x > snap_delta_y {
    //     draw_line(x, y, x2, y2_snap, 2.0, RED);
    // }
    // if snap_delta_x < snap_delta_y {
    //     draw_line(x, y, x1_snap, y2, 2.0, RED);
    // }

    draw_line(x, y, x2, y2, 2.0, RED);
}

fn snap_x(pixel_coordinate: f32, rotation_angle: f32) -> f32 {
    if rotation_angle.cos() >= 0.0 {
        (pixel_coordinate / TILESIZE as f32).ceil() * TILESIZE as f32
    } else if rotation_angle.cos() < 0.0 {
        (pixel_coordinate / TILESIZE as f32).floor() * TILESIZE as f32
    } else {
        unreachable!()
    }
}
fn snap_y(pixel_coordinate: f32, rotation_angle: f32) -> f32 {
    if rotation_angle.sin() > 0.0 {
        (pixel_coordinate / TILESIZE as f32).ceil() * TILESIZE as f32
    } else if rotation_angle.sin() <= 0.0 {
        (pixel_coordinate / TILESIZE as f32).floor() * TILESIZE as f32
    } else {
        unreachable!()
    }
}
fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // distance formula
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
fn has_wall_at(x_pixel_coordinate: f32, y_pixel_coordinate: f32) -> bool {
    GRID[(y_pixel_coordinate / TILESIZE as f32).floor() as usize]
        [(x_pixel_coordinate / TILESIZE as f32).floor() as usize]
        == 1
}
