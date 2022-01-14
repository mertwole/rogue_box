use crate::common::asset_manager::AssetId;
use crate::common::math::*;

#[derive(Clone)]
pub struct Sprite {
    pub texture : AssetId,
    pub position : Vec2,
    pub scale : Vec2,
    pub rotation : f32, 
}

impl Sprite {
    pub fn new(texture : AssetId) -> Sprite {
        Sprite {
            texture,
            position : Vec2::zero(),
            scale : Vec2::new(1.0, 1.0),
            rotation : 0.0
        }
    }
}