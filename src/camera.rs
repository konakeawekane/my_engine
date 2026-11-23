use crate::math::Vec;
pub struct Camera{
    pub position: Vec,
    pub rotation: Vec,
    pub h_fov : f32,
    pub v_fov : f32,
    pub x_scale : f32,
    pub y_scale : f32
}

impl Camera{
    pub fn new(position: Vec, rotation: Vec, h_fov: f32, aspect_ratio: f32) -> Self {
        let h_rad = h_fov.to_radians();
        let v_rad = 2.0 * ((h_rad * 0.5).tan() / aspect_ratio).atan();

        Self { 
            position: position,
            rotation: rotation,
            h_fov: h_fov,
            v_fov: v_rad.to_degrees(),
            x_scale: 1.0 / (h_rad * 0.5).tan(),
            y_scale: 1.0 / (v_rad * 0.5).tan()
        }
    }

    pub fn translate(&mut self, movement: Vec){
        self.position.x += movement.x;
        self.position.y += movement.y;
        self.position.z += movement.z;
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32, roll: f32){
        self.rotation.x += pitch;
        self.rotation.y += yaw;
        self.rotation.z += roll;
    }

    pub fn set_zoom(&mut self, zoom: f32){
        self.h_fov = self.h_fov * zoom;
        self.v_fov = self.v_fov * zoom;
    }

    pub fn set_fov(&mut self, h_fov: f32, aspect_ratio: f32){
        let h_rad = h_fov.to_radians();
        let v_rad = 2.0 * ((h_rad * 0.5).tan() / aspect_ratio).atan();

        self.h_fov = h_fov;
        self.v_fov = v_rad.to_degrees();

        self.x_scale = 1.0 / (h_rad * 0.5).tan();
        self.y_scale = 1.0 / (v_rad * 0.5).tan();
    }

    pub fn forward(&self) -> Vec{
        let yaw = self.rotation.y.to_radians();

        Vec{
            x: yaw.sin(),
            y: 0.0,
            z: yaw.cos()
        }
    }

    pub fn right(&self) -> Vec{
        let yaw = self.rotation.y.to_radians();

        Vec{
            x: yaw.cos(),
            y: 0.0,
            z: -yaw.sin()
        }
    }

    pub fn up(&self) -> Vec{
        Vec{
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
    }
}