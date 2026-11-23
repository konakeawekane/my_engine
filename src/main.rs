mod camera;
mod obj;
mod math;
mod color;
mod pen;

use minifb::{Key, Window, WindowOptions, ScaleMode};
use camera::Camera;
use color::Color;
use pen::Pen;
use obj::{Tri, Obj};

const WIDTH: usize = 1080;
const HEIGHT: usize = 720;
const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

const DRAWER: Pen = Pen::new(HEIGHT, WIDTH);

const RED: Color = Color::new(255,0,0);
const BLACK: Color = Color::new(0,0,0);

const VERTS: [math::Vec; 38] = [
    math::Vec::new(-1.713928, -1.713928, 1.713928),
    math::Vec::new(-1.713928, 1.713928, 1.713928),
    math::Vec::new(-1.713928, -1.713928, -1.713928),
    math::Vec::new(-1.713928, 1.713928, -1.713928),
    math::Vec::new(1.713928, -1.713928, 1.713928),
    math::Vec::new(1.713928, 1.713928, 1.713928),
    math::Vec::new(1.713928, -1.713928, -1.713928),
    math::Vec::new(1.713928, 1.713928, -1.713928),
    math::Vec::new(-1.369157, -1.711121, 1.573181),
    math::Vec::new(-1.369157, -1.008275, 1.573181),
    math::Vec::new(-1.369157, -1.711121, 0.870335),
    math::Vec::new(-1.369157, -1.008275, 0.870335),
    math::Vec::new(1.369157, -1.711121, 1.573181),
    math::Vec::new(1.369157, -1.008275, 1.573181),
    math::Vec::new(1.369157, -1.711121, 0.870335),
    math::Vec::new(1.369157, -1.008275, 0.870335),
    math::Vec::new(1.25, 1.326855, 0.0),
    math::Vec::new(0.875, 1.543361, 0.0),
    math::Vec::new(0.875, 1.110349, 0.0),
    math::Vec::new(0.625, 1.326855, -1.082532),
    math::Vec::new(0.4375, 1.543361, -0.757772),
    math::Vec::new(0.4375, 1.110349, -0.757772),
    math::Vec::new(-0.625, 1.326855, -1.082532),
    math::Vec::new(-0.4375, 1.543361, -0.757772),
    math::Vec::new(-0.4375, 1.110349, -0.757772),
    math::Vec::new(-1.25, 1.326855, -0.0),
    math::Vec::new(-0.875, 1.543361, -0.0),
    math::Vec::new(-0.875, 1.110349, -0.0),
    math::Vec::new(-0.625, 1.326855, 1.082532),
    math::Vec::new(-0.4375, 1.543361, 0.757772),
    math::Vec::new(-0.4375, 1.110349, 0.757772),
    math::Vec::new(0.625, 1.326855, 1.082532),
    math::Vec::new(0.4375, 1.543361, 0.757772),
    math::Vec::new(0.4375, 1.110349, 0.757772),
    math::Vec::new(-0.97872, -1.716204, -1.440551),
    math::Vec::new(-0.612893, -1.716204, -0.806921),
    math::Vec::new(-1.344546, -1.716204, -0.806921),
    math::Vec::new(-0.97872, -0.871364, -1.018131),
];

const TRIS: [Tri; 64] = [
    Tri::new(0, 1, 3),
    Tri::new(0, 3, 2),
    Tri::new(2, 3, 7),
    Tri::new(2, 7, 6),
    Tri::new(6, 7, 5),
    Tri::new(6, 5, 4),
    Tri::new(4, 5, 1),
    Tri::new(4, 1, 0),
    Tri::new(2, 6, 4),
    Tri::new(2, 4, 0),
    Tri::new(7, 3, 1),
    Tri::new(7, 1, 5),
    Tri::new(8, 9, 11),
    Tri::new(8, 11, 10),
    Tri::new(10, 11, 15),
    Tri::new(10, 15, 14),
    Tri::new(14, 15, 13),
    Tri::new(14, 13, 12),
    Tri::new(12, 13, 9),
    Tri::new(12, 9, 8),
    Tri::new(10, 14, 12),
    Tri::new(10, 12, 8),
    Tri::new(15, 11, 9),
    Tri::new(15, 9, 13),
    Tri::new(16, 19, 20),
    Tri::new(16, 20, 17),
    Tri::new(17, 20, 21),
    Tri::new(17, 21, 18),
    Tri::new(18, 21, 19),
    Tri::new(18, 19, 16),
    Tri::new(19, 22, 23),
    Tri::new(19, 23, 20),
    Tri::new(20, 23, 24),
    Tri::new(20, 24, 21),
    Tri::new(21, 24, 22),
    Tri::new(21, 22, 19),
    Tri::new(22, 25, 26),
    Tri::new(22, 26, 23),
    Tri::new(23, 26, 27),
    Tri::new(23, 27, 24),
    Tri::new(24, 27, 25),
    Tri::new(24, 25, 22),
    Tri::new(25, 28, 29),
    Tri::new(25, 29, 26),
    Tri::new(26, 29, 30),
    Tri::new(26, 30, 27),
    Tri::new(27, 30, 28),
    Tri::new(27, 28, 25),
    Tri::new(28, 31, 32),
    Tri::new(28, 32, 29),
    Tri::new(29, 32, 33),
    Tri::new(29, 33, 30),
    Tri::new(30, 33, 31),
    Tri::new(30, 31, 28),
    Tri::new(31, 16, 17),
    Tri::new(31, 17, 32),
    Tri::new(32, 17, 18),
    Tri::new(32, 18, 33),
    Tri::new(33, 18, 16),
    Tri::new(33, 16, 31),
    Tri::new(34, 37, 35),
    Tri::new(34, 35, 36),
    Tri::new(35, 37, 36),
    Tri::new(36, 37, 34),
];


const OBJ: obj::Obj<38,64> = Obj::new(
    VERTS,
    TRIS,
    math::Vec::new(0.0,0.0,1.0), 
    math::Vec::new(0.0,0.0,0.0),
    math::Vec::new(1.0, 1.0, 1.0)
);

fn main() {
    let mut cam = Camera::new(
        math::Vec::new(0.0,0.0,0.0),
        math::Vec::new(0.0,0.0,0.0),
        300.0,
        200.0
    );

    cam.set_fov(80.0, ASPECT_RATIO);

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

    let mut forward_input;
    let mut right_input;
    let mut up_input;
    let mut horizontal_look_input;
    let mut vertical_look_input;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // screen clear
        for pixel in &mut buffer{
            *pixel = BLACK.to_u32();
        }

        for triangle in OBJ.tris{
            let mut vert1 = OBJ.verts[triangle.id1];
            let mut vert2 = OBJ.verts[triangle.id2];
            let mut vert3 = OBJ.verts[triangle.id3];

            // __first find the world position__

            // scale in local space
            vert1 = math::Vec::scale_vec(vert1, OBJ.scale);
            vert2 = math::Vec::scale_vec(vert2, OBJ.scale);
            vert3 = math::Vec::scale_vec(vert3, OBJ.scale);

            // rotate in local space
            vert1 = math::Vec::rotate_around_orgin(vert1, OBJ.rotation);
            vert2 = math::Vec::rotate_around_orgin(vert2, OBJ.rotation);
            vert3 = math::Vec::rotate_around_orgin(vert3, OBJ.rotation);

            // move to world postion
            vert1 = math::Vec::add(vert1, OBJ.position);
            vert2 = math::Vec::add(vert2, OBJ.position);
            vert3 = math::Vec::add(vert3, OBJ.position);
            
            // __Next rotate into camera space via perspective projection__

            vert1 = math::Vec::project(
                vert1, 
                cam.position, 
                cam.rotation, 
                cam.x_scale, 
                cam.y_scale
            );

            vert2 = math::Vec::project(
                vert2,
                cam.position, 
                cam.rotation, 
                cam.x_scale, 
                cam.y_scale
            );

            vert3 = math::Vec::project(
                vert3, 
                cam.position, 
                cam.rotation, 
                cam.x_scale, 
                cam.y_scale
            );

            if(!(vert1.z < 1.0 || vert2.z < 1.0 || vert3.z < 1.0)){
                DRAWER.draw_line(&mut buffer, vert1.x, vert1.y, vert2.x, vert2.y, RED.to_u32());
                DRAWER.draw_line(&mut buffer, vert2.x, vert2.y, vert3.x, vert3.y, RED.to_u32());
                DRAWER.draw_line(&mut buffer, vert3.x, vert3.y, vert1.x, vert1.y, RED.to_u32());
            }

        }

        forward_input = 0.0;
        right_input = 0.0;
        up_input = 0.0;
        horizontal_look_input = 0.0;
        vertical_look_input = 0.0;

        if window.is_key_down(Key::W){
            forward_input = 0.03;
        }

        if window.is_key_down(Key::A){
            right_input = -0.03;
        }

        if window.is_key_down(Key::S){
            forward_input = -0.03;
        }

        if window.is_key_down(Key::D){
            right_input = 0.03;
        }

        if window.is_key_down(Key::Q){
            up_input = -0.03;
        }

        if window.is_key_down(Key::E){
            up_input = 0.03;
        }

        if window.is_key_down(Key::Left){
            horizontal_look_input = -1.0;
        }

        if window.is_key_down(Key::Right){
            horizontal_look_input = 1.0;
        }

        if window.is_key_down(Key::Up){
            vertical_look_input = -1.0;
        }

        if window.is_key_down(Key::Down){
            vertical_look_input = 1.0;
        }

        cam.translate(math::Vec::scale(cam.forward(), forward_input));
        cam.translate(math::Vec::scale(cam.right(), right_input));
        cam.translate(math::Vec::scale(cam.up(), up_input));

        cam.rotate(horizontal_look_input, vertical_look_input, 0.0);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}
