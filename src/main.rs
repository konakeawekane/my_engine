mod camera;
mod obj;
mod math;
mod color;

use minifb::{Key, Window, WindowOptions};
use camera::Camera;
use color::Color;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const RED: Color = Color::new(255,0,0);

fn main() {
    let mut cam = Camera::new(
        math::Vec::new(0.0,0.0,0.0),
        math::Vec::new(0.0,0.0,0.0),
        80.0);

    cam.rotate(1.0, -10.0, 0.0);
    cam.translate(-10.0, -2.0, 1.0);
    cam.set_zoom(90.0);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    let x = 30;
    let y = 30;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        draw_line(&mut buffer, WIDTH, 50.0, 20.0, 200.0, 150.0, RED.to_u32());

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}

fn set_pixel(
    buffer: &mut Vec<u32>,
    x: usize,
    y: usize,
    color: u32,
) {
    buffer[x + y * WIDTH] = color;
}

fn draw_line(
    buffer: &mut Vec<u32>,
    width: usize,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: u32,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;

    let steps = dx.abs().max(dy.abs());

    let x_increment = dx / steps;
    let y_increment = dy / steps;

    let mut x = x0;
    let mut y = y0;

    for _ in 0..steps as usize {
        let index = x as usize + y as usize * width;

        buffer[index] = color;

        x += x_increment;
        y += y_increment;
    }
}
