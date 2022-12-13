use crate::game::common::asset_manager::AssetId;

use super::*;

#[derive(Clone)]
pub struct CraftedItem {
    item: Item,
}

impl CraftedItem {
    pub fn new(item: Item) -> CraftedItem {
        CraftedItem { item }
    }

    pub fn get_id(&self) -> ItemId {
        self.item.get_id()
    }

    pub fn get_sprite_asset_id(&self) -> AssetId {
        self.item.sprite.texture
    }
}
