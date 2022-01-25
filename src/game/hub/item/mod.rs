extern crate serde_json;

use crate::game::common::asset_manager::AssetManager;
use crate::game::game_entity::*;
use crate::game::common::math::Vec2;
use crate::game::renderer::{Renderer, Sprite};

mod item_factory;
mod transported_item;
pub use item_factory::*;
pub use transported_item::*;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct ItemId(u64);

#[derive(Clone)]
struct ItemMovement {
    from : Vec2,
    to : Vec2,
    tick_id : u32
}

pub struct Item {
    id : ItemId,
    sprite : Sprite,
    
    movement : Option<ItemMovement>
}



impl Item {
    fn new_error() -> Item {
        let texture = AssetManager::get_asset_id("error_fallbacks/texture.png");
        let sprite = Sprite::new(texture);
        Item { 
            id : ItemId(0), 
            sprite,
            movement : None
        }
    }

    pub fn from_json_value(value : &serde_json::Value) -> Item {
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
                    sprite,
                    movement : None
                };
                
                new_item
            }
            _ => { 
                log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure"); 
                Self::new_error()
            }
        }
    }

    pub fn get_id(&self) -> ItemId {
        self.id
    }

    pub fn set_movement(&mut self, from : Vec2, to : Vec2, tick_id : u32) {
        self.movement = Some(ItemMovement { from, to, tick_id });
    }
}

impl GameEntity for Item {
    fn update(&mut self, parameters : &UpdateParameters) {
        match &self.movement {
            None => {
                self.sprite.local_transform.translation = Vec2::zero();
            }
            Some(movement) => {
                if movement.tick_id + 1 == parameters.last_tick_id {
                    let interpolation = parameters.from_last_tick / crate::game::TICK_PERIOD;
                    self.sprite.local_transform.translation = movement.from + (movement.to - movement.from) * interpolation;
                } else {
                    self.sprite.local_transform.translation = movement.to;
                }
            }
        }
    }

    fn tick(&mut self, tick_id : u32) {

    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        renderer.queue_render_sprite(self.sprite.clone(), transform);
    }
}

impl Clone for Item { 
    fn clone(&self) -> Item {
        Item { 
            id : self.id, 
            sprite : self.sprite.clone(),
            movement : self.movement.clone()
        }
    }
}

