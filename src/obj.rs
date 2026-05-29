use crate::math::Vec;

pub struct Tri{
    pub id1: usize,
    pub id2: usize,
    pub id3: usize
}

pub struct Obj<const VERT_COUNT: usize, const TRIS_COUNT: usize>{
    pub verts: [Vec; VERT_COUNT],
    pub tris: [Tri; TRIS_COUNT],
    pub position: Vec,
    pub rotation: Vec,
    pub scale: Vec
}

impl Tri{
    pub const fn new(v1: usize, v2: usize, v3: usize) -> Self{
        Self{
            id1: v1,
            id2: v2,
            id3: v3
        }
    }
}

impl<const VERT_COUNT: usize, const TRIS_COUNT: usize> Obj<VERT_COUNT, TRIS_COUNT>{
    pub const fn new(verts: [Vec; VERT_COUNT], tris: [Tri; TRIS_COUNT], position: Vec, rotation: Vec, scale: Vec) -> Self{
        Self{
            verts: verts,
            tris: tris,
            position: position,
            rotation: rotation,
            scale: scale
        }
    }
}