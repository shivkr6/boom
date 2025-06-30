use macroquad::prelude::*;
use macroquad::window::Conf;
use std::array;
use std::f32::consts::PI;

const TILESIZE: i32 = 96;
const WALL_HEIGHT: i32 = TILESIZE;

const ROWS: usize = 10;
const COLS: usize = 15;

const WINDOW_HEIGHT: i32 = ROWS as i32 * TILESIZE;
const WINDOW_WIDTH: i32 = COLS as i32 * TILESIZE;

const FOV: f32 = 60.0 * (PI / 180.0);
const COS_HALF_FOV: f32 = 0.866_025_4;
const TAN_HALF_FOV: f32 = 0.577_350_26;

const CAMERA_RAY_LENGTH: f32 = 300.0;
const FOV_BOUNDARY_RAY_LENGTH: f32 = CAMERA_RAY_LENGTH / COS_HALF_FOV; // This magic number is (FOV/2).cos()
const PROJECTION_PLANE_LENGTH: f32 = 2.0 * CAMERA_RAY_LENGTH * TAN_HALF_FOV;

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
    let mut rays: [(f32, String); NUM_RAYS as usize] = array::from_fn(|_| (0.0, String::new()));
    loop {
        clear_background(BLACK);
        // init_grid();
        init_player(&mut x, &mut y, &mut rotation_angle);
        draw_rays(x, y, rotation_angle, &mut rays);
        render_game(&rays);
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

// fn init_grid() {
//     for (i, row) in GRID.iter().enumerate() {
//         for (j, &cell) in row.iter().enumerate() {
//             // Coordinates in pixels
//             let tile_x: f32 = j as f32 * TILESIZE as f32;
//             let tile_y: f32 = i as f32 * TILESIZE as f32;
//
//             if cell == 0 {
//                 draw_rectangle(
//                     tile_x,
//                     tile_y,
//                     (TILESIZE - 1) as f32,
//                     (TILESIZE - 1) as f32,
//                     WHITE,
//                 );
//             } else if cell == 1 {
//                 draw_rectangle(
//                     tile_x,
//                     tile_y,
//                     (TILESIZE - 1) as f32,
//                     (TILESIZE - 1) as f32,
//                     BLACK,
//                 );
//             }
//         }
//     }
// }

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

    // draw_circle(*x, *y, radius, RED);
}

fn draw_rays(x: f32, y: f32, rotation_angle: f32, rays: &mut [(f32, String); NUM_RAYS as usize]) {
    let mut ray_angle: f32 = rotation_angle - (FOV / 2.0);
    let gap_angle: f32 = FOV / (NUM_RAYS - 1) as f32;

    for i in 0..NUM_RAYS {
        // while ray_angle <= end_angle {
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
                // draw_line(x2, y2, x2_snap, y2_snap, 2.0, RED);
                rays[i as usize] = (distance(x, y, x2_snap, y2_snap), "gray".to_string());

                if has_wall_at(x2_snap, y2_snap) {
                    break;
                }
                x2 = x2_snap;
                y2 = y2_snap;
            }
            if len_snap_1 < len_snap_2 {
                // draw_line(x2, y2, x1_snap, y1_snap, 2.0, RED);
                rays[i as usize] = (distance(x, y, x1_snap, y1_snap), "white".to_string());

                if has_wall_at(x1_snap, y1_snap) {
                    break;
                }
                x2 = x1_snap;
                y2 = y1_snap;
            }
        }
        ray_angle += gap_angle;
    }
    // render_fov_lines(x, y, rotation_angle);
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
// fn render_fov_lines(x: f32, y: f32, rotation_angle: f32) {
//     let start_angle: f32 = rotation_angle - FOV / 2.0;
//     let end_angle: f32 = rotation_angle + FOV / 2.0;
//
//     let x1 = x + rotation_angle.cos() * 100.0;
//     let y1 = y + rotation_angle.sin() * 100.0;
//     let screen_distance: f32 = distance(x, y, x1, y1);
//     let side_ray_distance: f32 = screen_distance / (FOV / 2.0).cos();
//
//     let x2 = x + start_angle.cos() * side_ray_distance;
//     let y2 = y + start_angle.sin() * side_ray_distance;
//
//     let x3 = x + end_angle.cos() * side_ray_distance;
//     let y3 = y + end_angle.sin() * side_ray_distance;
//
//     draw_line(x, y, x1, y1, 5.0, GREEN);
//     draw_line(x, y, x2, y2, 5.0, GREEN);
//     draw_line(x, y, x3, y3, 5.0, GREEN);
//
//     draw_line(x2, y2, x3, y3, 5.0, GREEN);
// }

fn render_game(rays: &[(f32, String); NUM_RAYS as usize]) {
    for (i, (ray_len, ray_color)) in rays.iter().enumerate() {
        let line_height = (WALL_HEIGHT as f32 / ray_len) * PROJECTION_PLANE_LENGTH;

        let draw_begin = (WINDOW_HEIGHT as f32 / 2.0) - (line_height / 2.0);

        if ray_color == "white" {
            draw_rectangle(
                i as f32 * RES as f32,
                draw_begin,
                RES as f32,
                line_height,
                WHITE,
            );
        } else {
            draw_rectangle(
                i as f32 * RES as f32,
                draw_begin,
                RES as f32,
                line_height,
                GRAY,
            );
        }
    }
}
