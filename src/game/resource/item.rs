use std::collections::{HashMap, hash_map::DefaultHasher};
use std::rc::Rc;
use std::hash::{Hash, Hasher};

extern crate serde_json;

use super::Resource;
use crate::common::asset_manager::{AssetManager, AssetId};
use crate::game::game_entity::GameEntity;
use crate::common::math::Vec2;
use crate::game::renderer::{Renderer, Sprite};

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct ItemId(u64);

pub struct Item {
    id : ItemId,
    sprite : Sprite
}

impl Item {
    fn new_error() -> Item {
        let texture = AssetManager::get_asset_id("error_fallbacks/texture.png");
        let sprite = Sprite::new(texture);
        Item { 
            id : ItemId(0), 
            sprite
        }
    }

    fn from_json_value(value : &serde_json::Value) -> Item {
        match value {
            serde_json::Value::Object(item_obj) => {
                let err_name = &String::from("error");
                let name = match item_obj.get("name") {
                    Some(serde_json::Value::String(name)) => { name }
                    _ => { 
                        log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(wrong item format)");
                        &err_name
                    }
                };

                let err_tex_path = &String::from("error_fallbacks/texture.png");
                let tex_path = match item_obj.get("texture") {
                    Some(serde_json::Value::String(path)) => { path }
                    _ => { 
                        log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(wrong item format)");
                        &err_tex_path
                    }
                };

                let sprite = Sprite::new(AssetManager::get_asset_id(tex_path.as_str()));
                let new_item = Item { 
                    id : ItemFactory::get_item_id_by_name(name.as_str()),
                    sprite
                };
                
                new_item
            }
            _ => { 
                log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure"); 
                Self::new_error()
            }
        }
    }

    pub fn set_position(&mut self, position : Vec2) {
        self.sprite.position = position;
    }
}

impl GameEntity for Item {
    fn update(&mut self, delta_time : f32) {

    }

    fn tick(&mut self) {

    }

    fn render(&mut self, renderer : &mut Renderer) {
        renderer.queue_render_sprite(&self.sprite);
    }
}

impl Clone for Item { 
    fn clone(&self) -> Item {
        Item { 
            id : self.id, 
            sprite : self.sprite.clone()
        }
    }
}

impl Resource for Item { }

pub struct ItemFactory {
    items : HashMap<ItemId, Item>
}

impl ItemFactory {
    pub fn new(json : Rc<str>) -> ItemFactory {
        let mut items = HashMap::new();

        let items_arr = serde_json::from_str(json.as_ref()).unwrap_or_else(|e| { 
            log::error!("Item dictionary haven't been succesfully loaded : {}", e);
            serde_json::Value::Array(Vec::new())
        });

        match items_arr {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    let new_item = Item::from_json_value(&item);
                    items.insert(new_item.id, new_item);
                }
            }
            _ => { log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(the top-level must be an array)"); }
        }

        log::info!("{} items are loaded", items.len());
        ItemFactory { items }
    }

    pub fn create_item(&self, id : ItemId) -> Item {
        match self.items.get(&id) {
            Some(item) => { item.clone() }
            None => {
                log::error!("There's no such item {:#034x}", id.0);
                Item::new_error()
            }
        }
    }

    pub fn get_item_id_by_name(name : &str) -> ItemId {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        ItemId(hasher.finish())
    }
}