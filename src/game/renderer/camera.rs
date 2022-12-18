use crate::game::common::math::{IVec2, Rect, Vec2};

#[derive(Clone)]
pub struct CameraProperties {
    pub offset: Vec2,
    pub cell_size: f32,
}

impl Default for CameraProperties {
    fn default() -> CameraProperties {
        CameraProperties {
            cell_size: 32.0,
            offset: Vec2::zero(),
        }
    }
}

pub struct Camera {
    properties: CameraProperties,
    resolution: IVec2,
}

impl Camera {
    pub fn new(resolution: IVec2) -> Camera {
        Camera {
            properties: CameraProperties::default(),
            resolution,
        }
    }

    pub fn set_properties(&mut self, properties: CameraProperties) {
        self.properties = properties;
    }

    pub fn set_resolution(&mut self, resolution: IVec2) {
        self.resolution = resolution;
    }

    // pub fn set_offset(&mut self, offset: Vec2) {
    //     self.properties.offset = offset;
    // }

    // pub fn set_cell_size(&mut self, size: f32) {
    //     self.properties.cell_size = size;
    // }

    pub fn get_cell_size(&self) -> f32 {
        self.properties.cell_size
    }

    pub fn world_to_screen_space(&self, world: Vec2) -> IVec2 {
        let mut res = ((world - self.properties.offset) * self.properties.cell_size
            + self.resolution.to_vec2() * 0.5)
            .to_ivec2();
        res.y = self.resolution.y - res.y;
        res
    }

    pub fn get_bounds(&self) -> Rect {
        let resolution_cells = self.resolution.to_vec2() / self.properties.cell_size;
        let mut bounds = Rect::zero();
        bounds.min = self.properties.offset - resolution_cells * 0.5;
        bounds.max = self.properties.offset + resolution_cells * 0.5;
        bounds
    }
}
