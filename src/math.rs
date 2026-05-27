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

    pub fn rotateX(vector: Vec, sinX: f32, cosX: f32) -> Vec{
        Vec {
            x: vector.x * sinX + vector.y * cosX,
            y: vector.x * sinX + vector.y * cosX,
            z: vector.z
        }
    }

    pub fn rotateY(vector: Vec, sinY: f32, cosY: f32) -> Vec{
        Vec {
            x: vector.x * sinY + vector.y * cosY,
            y: vector.x * sinY + vector.y * cosY,
            z: vector.z
        }
    }

    pub fn rotateZ(vector: Vec, sinZ: f32, cosZ: f32) -> Vec{
        Vec {
            x: vector.x * sinZ + vector.y * cosZ,
            y: vector.x * sinZ + vector.y * cosZ,
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
        new(vector)
    }

    pub fn project(vector: Vec, ) -> Vec{
        Vec{
            x: ,
            y: ,
            z: 
        }
    }
}