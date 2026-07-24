use crate::math::Vec;
pub struct Camera{
    pub position: Vec,
    pub rotation: Vec,
    pub h_fov : f32,
    pub v_fov : f32
}

impl Camera{
    pub fn new(position: Vec, rotation: Vec, h_fov: f32, v_fov: f32) -> Self {
        Self { 
            position: position,
            rotation: rotation,
            h_fov: h_fov,
            v_fov: v_fov
        }
    }

    pub fn translate(&mut self, movement: Vec){
        self.position.x += movement.x;
        self.position.y += movement.y;
        self.position.z += movement.z;
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32, roll: f32){
        self.rotation.x += yaw;
        self.rotation.y += pitch;
        self.rotation.z += roll;
    }

    pub fn set_zoom(&mut self, zoom: f32){
        self.h_fov = self.h_fov * zoom;
        self.v_fov = self.v_fov * zoom;
    }

    pub fn set_fov(&mut self, h_fov: f32, aspect_ratio: f32){
        self.h_fov = h_fov;
        self.v_fov = 2.0 * ((h_fov / 2.0).tan() / aspect_ratio).atan()
    }

    pub fn forward(&self) -> Vec{
        Vec{
            x: self.rotation.x.sin(),
            y: 0.0,
            z: self.rotation.x.cos()
        }
    }

    pub fn right(&self) -> Vec{
        Vec{
            x: self.rotation.x.cos(),
            y: 0.0,
            z: -self.rotation.x.sin()
        }
    }

    pub fn up(&self) -> Vec{
        Vec{
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }
}