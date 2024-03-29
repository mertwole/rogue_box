extern crate serde_json;

use crate::game::common::{
    asset_manager::{AssetId, AssetManager},
    math::Vec2,
};
use crate::game::game_entity::*;
use crate::game::renderer::{Renderer, Sprite};

mod item_factory;
mod transported_item;
pub use item_factory::*;
pub use transported_item::*;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct ItemId(u64);

#[derive(Clone)]
pub struct Item {
    id: ItemId,
    sprite: Sprite,
}

impl Item {
    // DEBUG
    pub fn new_error() -> Item {
        let texture = AssetManager::get_asset_id("error_fallbacks/texture.png");
        let sprite = Sprite::new(texture);
        Item {
            id: ItemId(0),
            sprite,
        }
    }

    pub fn from_json_value(value: &serde_json::Value) -> Item {
        match value {
            serde_json::Value::Object(item_obj) => {
                let err_name = &String::from("error");
                let name = match item_obj.get("name") {
                    Some(serde_json::Value::String(name)) => name,
                    _ => {
                        log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(wrong item format)");
                        &err_name
                    }
                };

                let err_tex_path = &String::from("error_fallbacks/texture.png");
                let tex_path = match item_obj.get("texture") {
                    Some(serde_json::Value::String(path)) => path,
                    _ => {
                        log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(wrong item format)");
                        &err_tex_path
                    }
                };

                let sprite = Sprite::new(AssetManager::get_asset_id(tex_path.as_str()));
                Item {
                    id: ItemFactory::get_item_id_by_name(name.as_str()),
                    sprite,
                }
            }
            _ => {
                log::error!(
                    "Item dictionary haven't been succesfully loaded : wrong JSON file structure"
                );
                Self::new_error()
            }
        }
    }

    pub fn get_id(&self) -> ItemId {
        self.id
    }

    pub fn get_sprite_asset_id(&self) -> AssetId {
        self.sprite.texture
    }
}
