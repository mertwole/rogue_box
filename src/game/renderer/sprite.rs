use crate::common::asset_manager::AssetId;

use super::SpriteTransform;

#[derive(Clone)]
pub struct Sprite {
    pub texture : AssetId,
    pub local_transform : SpriteTransform
}

impl Sprite {
    pub fn new(texture : AssetId) -> Sprite {
        Sprite {
            texture,
            local_transform : SpriteTransform::default()
        }
    }
}