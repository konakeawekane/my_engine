use crate::math::Vec;
pub struct Camera{
    pub position: Vec,
    pub rotation: Vec,
    pub fov : f32
}

impl Camera{
    pub fn new(position: Vec, rotation: Vec, fov: f32) -> Self {
        Self { 
            position: position,
            rotation: rotation,
            fov: fov 
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

    pub fn set_zoom(&mut self, fov:f32){
        self.fov = fov;
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
}