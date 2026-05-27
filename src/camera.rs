
pub struct Camera{
    pub x : f32,
    pub y :  f32,
    pub z : f32,
    pub yaw : f32,
    pub pitch : f32,
    pub roll : f32,
    pub fov : f32
}

impl Camera{
    pub fn new(x:f32,y:f32,z:f32,yaw:f32,pitch:f32,roll:f32,fov:f32) -> Self {
        Self { 
            x: x,
            y: y,
            z: z,
            yaw: yaw,
            pitch: pitch,
            roll: roll,
            fov: fov 
        }
    }

    pub fn set_pos(&mut self, x:f32, y:f32, z:f32){
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn set_rot(&mut self, yaw:f32, pitch:f32, roll:f32){
        self.yaw = yaw;
        self.pitch = pitch;
        self.roll = roll;
    }

    pub fn set_zoom(&mut self, fov:f32){
        self.fov = fov;
    }
}