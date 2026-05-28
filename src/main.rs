mod camera;
mod obj;
mod math;
mod color;
mod pen;

use minifb::{Key, Window, WindowOptions, ScaleMode};
use camera::Camera;
use color::Color;
use pen::Pen;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const drawer: Pen = Pen::new(HEIGHT, WIDTH);

const RED: Color = Color::new(255,0,0);
const BLACK: Color = Color::new(0,0,0);

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
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        }
    )
    .unwrap();

    window.set_target_fps(60);

    let mut x: f32 = 150.0;
    let mut y: f32 = 150.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // screen clear
        for pixel in &mut buffer{
            *pixel = BLACK.to_u32();
        }

        drawer.draw_line(&mut buffer, 50.0, 20.0, x, y, RED.to_u32());

        if(window.is_key_down(Key::W)){
            y -= 1.0;
        }

        if(window.is_key_down(Key::A)){
            x -= 1.0;
        }

        if(window.is_key_down(Key::S)){
            y += 1.0;
        }

        if(window.is_key_down(Key::D)){
            x += 1.0;
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}

