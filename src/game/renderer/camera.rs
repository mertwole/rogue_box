use crate::common::math::{Vec2, IVec2};

pub struct Camera {
    cell_size : f32,
    offset : Vec2,
    resolution : IVec2
}

impl Camera {
    pub fn new(resolution : IVec2) -> Camera {
        Camera {
            cell_size : 32.0,
            offset : Vec2::zero(),
            resolution
        }
    }

    pub fn set_offset(&mut self, offset : Vec2) { self.offset = offset; }

    pub fn set_cell_size(&mut self, size : f32) { self.cell_size = size; }

    pub fn get_cell_size(&self) -> f32 { self.cell_size }

    pub fn world_to_screen_space(&self, world : Vec2) -> IVec2 {
        let mut res = ((world - self.offset) * self.cell_size + self.resolution.to_vec2() * 0.5).to_ivec2();
        res.y = self.resolution.y - res.y;
        res
    }
}