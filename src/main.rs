mod camera;
mod obj;
mod math;

use camera::Camera;

fn main() {
    let mut cam = Camera::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 80.0);

    cam.set_pos(10.0, 0.0, 0.0);

    cam.set_rot(5.0, 0.0, 0.0);

    cam.set_zoom(90.0);
}
