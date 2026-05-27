mod camera;
mod obj;
mod math;

use camera::Camera;

fn main() {
    let mut cam = Camera::new(
        math::Vec::new(0.0,0.0,0.0),
        math::Vec::new(0.0,0.0,0.0),
        80.0);

    cam.rotate(1.0, -10.0, 0.0);

    cam.translate(-10.0, -2.0, 1.0);

    cam.set_zoom(90.0);
}
