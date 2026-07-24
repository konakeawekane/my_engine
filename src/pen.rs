pub struct Pen{
    pub buffer_width: usize,
    pub buffer_height: usize
}

impl Pen{

    pub const fn new(buffer_height: usize, buffer_width: usize) -> Self{
        Self{
            buffer_height: buffer_height,
            buffer_width: buffer_width
        }
    }

    pub fn valid_buffer_index(&self, x: usize, y:usize) -> bool{

        x < self.buffer_width && y < self.buffer_height
    }

    pub fn set_pixel(
        &self,
        buffer: &mut Vec<u32>,
        x: usize,
        y: usize,
        color: u32,
    ) {
        // needs to be refactored outside of the set pixel check to increase performance
        if Self::valid_buffer_index(self, x, y){
            buffer[x + y * self.buffer_width] = color;
        }
    }


    // need to be refactored into a real DDA algorithm
    pub fn draw_line(
        &self,
        buffer: &mut Vec<u32>,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: u32,
    ) {
        let half_width = (self.buffer_width / 2) as f32;
        let half_height = (self.buffer_height / 2) as f32;

        let x0 = x0 * half_width + half_width;
        let y0 = y0 * half_height + half_height;
        let x1 = x1 * half_width + half_width;
        let y1 = y1 * half_height + half_height;

        let dx = x1 - x0;
        let dy = y1 - y0;

        let steps = dx.abs().max(dy.abs());

        let x_increment = dx / steps;
        let y_increment = dy / steps;

        let mut x = x0;
        let mut y = y0;

        for _ in 0..steps as usize {
            Self::set_pixel(self, buffer, x as usize, y as usize, color);

            x += x_increment;
            y += y_increment;
        }
    }
}