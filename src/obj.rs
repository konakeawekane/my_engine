use crate::math::Vec;

pub struct Tri{
    pub id1: i32,
    pub id2: i32,
    pub id3: i32
}

pub struct Obj<const nVerts: usize, const nTris: usize>{
    pub verts: [Vec; nVerts],
    pub tris: [Tri; nTris],
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

impl<const nVerts: usize, const nTris: usize> Obj<nVerts, nTris>{
    pub fn new(verts: [Vec; nVerts], tris: [Tri; nTris], position: Vec, rotation: Vec, scale: Vec) -> Self{
        Self{
            verts: verts,
            tris: tris,
            position: position,
            rotation: rotation,
            scale: scale
        }
    }
}