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

const DRAWER: Pen = Pen::new(HEIGHT, WIDTH);

const RED: Color = Color::new(255,0,0);
const BLUE: Color = Color::new(0,0,255);
const BLACK: Color = Color::new(0,0,0);

const VERTS: [math::Vec; 3] = [
    math::Vec::new(-1.0,-1.0,0.0),
    math::Vec::new(0.0,1.0,0.0),
    math::Vec::new(1.0,-1.0,0.0)
];

const TRIS: [Tri; 1] = [
    Tri::new(0,1,2)
];

const OBJ: obj::Obj<3,1> = Obj::new(
    VERTS,
    TRIS,
    math::Vec::new(0.0,0.0,100.0), 
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

            vert1 = math::Vec::project(vert1, cam.position, cam.rotation, cam.h_fov, cam.v_fov);
            vert2 = math::Vec::project(vert2, cam.position, cam.rotation, cam.h_fov, cam.v_fov);
            vert3 = math::Vec::project(vert3, cam.position, cam.rotation, cam.h_fov, cam.v_fov);

            if(!(vert1.z < 1.0 || vert2.z < 1.0 || vert3.z < 1.0)){
                DRAWER.draw_line(&mut buffer, vert1.x, vert1.y, vert2.x, vert2.y, RED.to_u32());
                DRAWER.draw_line(&mut buffer, vert2.x, vert2.y, vert3.x, vert3.y, RED.to_u32());
                DRAWER.draw_line(&mut buffer, vert3.x, vert3.y, vert1.x, vert1.y, RED.to_u32());
            }

        }

        DRAWER.draw_line(&mut buffer, 0.0, 0.0, 100.0, 100.0, BLUE.to_u32());

        forward_input = 0.0;
        right_input = 0.0;
        up_input = 0.0;
        horizontal_look_input = 0.0;

        if window.is_key_down(Key::W){
            forward_input = 1.0;
        }

        if window.is_key_down(Key::A){
            right_input = -1.0;
        }

        if window.is_key_down(Key::S){
            forward_input = -1.0;
        }

        if window.is_key_down(Key::D){
            right_input = 1.0;
        }

        if window.is_key_down(Key::Q){
            up_input = -1.0;
        }

        if window.is_key_down(Key::E){
            up_input = 1.0;
        }

        if window.is_key_down(Key::Left){
            horizontal_look_input = -0.01;
        }

        if window.is_key_down(Key::Right){
            horizontal_look_input = 0.01;
        }

        cam.translate(math::Vec::scale(cam.forward(), forward_input));
        cam.translate(math::Vec::scale(cam.right(), right_input));
        cam.translate(math::Vec::scale(cam.up(), up_input));

        cam.rotate(horizontal_look_input, 0.0, 0.0);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}
