use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Vec{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec{

    pub const fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x: x,
            y: y,
            z: z
        }
    }

    pub fn add(vector1: Vec, vector2: Vec) -> Vec{
        Vec {
            x: vector1.x + vector2.x,
            y: vector1.y + vector2.y,
            z: vector1.z + vector2.z 
        }
    }

    pub fn sub(vector1: Vec, vector2: Vec) -> Vec{
        Vec {
            x: vector1.x - vector2.x,
            y: vector1.y - vector2.y,
            z: vector1.z - vector2.z 
        }
    }

    pub fn scale(vector1: Vec, scale: f32) -> Vec{
        Vec {
            x: vector1.x * scale,
            y: vector1.y * scale,
            z: vector1.z * scale 
        }
    }

    pub fn rotate_x(vector: Vec, sin_x: f32, cos_x: f32) -> Vec{
        Vec {
            x: vector.x * cos_x + vector.z * sin_x,
            y: vector.y,
            z: vector.x * sin_x + vector.z * cos_x
        }
    }

    pub fn rotate_y(vector: Vec, sin_y: f32, cos_y: f32) -> Vec{
        Vec {
            x: vector.x,
            y: vector.y * cos_y + vector.z * sin_y,
            z: vector.z * cos_y + vector.y * sin_y
        }
    }

    pub fn rotate_z(vector: Vec, sin_z: f32, cos_z: f32) -> Vec{
        Vec {
            x: vector.x * cos_z + vector.y * sin_z,
            y: vector.y * sin_z + vector.x * cos_z,
            z: vector.z
        }
    }

    pub fn scale_vec(vector: Vec, size: Vec) -> Vec{
        Vec{
            x: vector.x * size.x,
            y: vector.y * size.y,
            z: vector.z * size.z
        }
    }

    pub fn rotate_around_orgin(vector: Vec, angle: Vec) -> Vec{
        let div = PI/180.0;
        let rad = Vec::new(angle.x * div, angle.y * div, angle.z * div);
        Self::rotate_z(
            Self::rotate_y(
                Self::rotate_x(
                        vector,
                        rad.x.sin(),
                        rad.x.cos()),
                rad.y.sin(),
                rad.y.cos()
            ),
            rad.z.sin(),
            rad.z.cos()
        )
    }

    pub fn project(vector: Vec, view_position: Vec, view_rotation: Vec, view_perspective: f32) -> Vec{
       let rotated = Self::rotate_around_orgin(Self::sub(vector, view_position), view_rotation);
       Vec{
            x: rotated.x * view_perspective / rotated.z,
            y: rotated.y * view_perspective / rotated.z,
            z: rotated.z,
       }
    }
}