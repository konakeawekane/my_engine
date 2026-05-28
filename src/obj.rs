use crate::math::Vec;

pub struct Tri{
    pub id1: i32,
    pub id2: i32,
    pub id3: i32
}

pub struct Obj<const VERT_COUNT: usize, const TRIS_COUNT: usize>{
    pub verts: [Vec; VERT_COUNT],
    pub tris: [Tri; TRIS_COUNT],
    pub position: Vec,
    pub rotation: Vec,
    pub scale: Vec
}

impl Tri{
    pub fn new(v1: i32, v2: i32, v3: i32) -> Self{
        Self{
            id1: v1,
            id2: v2,
            id3: v3
        }
    }
}

impl<const VERT_COUNT: usize, const TRIS_COUNT: usize> Obj<VERT_COUNT, TRIS_COUNT>{
    pub fn new(verts: [Vec; VERT_COUNT], tris: [Tri; TRIS_COUNT], position: Vec, rotation: Vec, scale: Vec) -> Self{
        Self{
            verts: verts,
            tris: tris,
            position: position,
            rotation: rotation,
            scale: scale
        }
    }

    pub fn edges(&self) -> [Vec; 1]{
        [Vec{
            x:1.0,y:1.0,z:1.0
        }]
    }
}