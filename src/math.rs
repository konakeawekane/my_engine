pub struct Vec{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec{

    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x: x,
            y: y,
            z: z
        }
    }

    pub fn rotateX(vector: Vec, sin_x: f32, cos_x: f32) -> Vec{
        Vec {
            x: vector.x * sin_x + vector.y * cos_x,
            y: vector.x * sin_x + vector.y * cos_x,
            z: vector.z
        }
    }

    pub fn rotateY(vector: Vec, sin_y: f32, cos_y: f32) -> Vec{
        Vec {
            x: vector.x * sin_y + vector.y * cos_y,
            y: vector.x * sin_y + vector.y * cos_y,
            z: vector.z
        }
    }

    pub fn rotateZ(vector: Vec, sin_z: f32, cos_z: f32) -> Vec{
        Vec {
            x: vector.x * sin_z + vector.y * cos_z,
            y: vector.x * sin_z + vector.y * cos_z,
            z: vector.z
        }
    }

    pub fn scale(vector: Vec, size: f32) -> Vec{
        Vec{
            x: vector.x * size,
            y: vector.y * size,
            z: vector.z * size
        }
    }

    pub fn rotate_around_orgin(vector: Vec, angles: Vec) -> Vec{
        vector
    }

    pub fn project(vector: Vec, ) -> Vec{
       vector
    }
}