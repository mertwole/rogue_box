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
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct TickId(u32);

pub struct Item {
    id : ItemId,
    sprite : Sprite,
    positions : HashMap<TickId, Vec2>,
    from_last_tick : f32,
    last_tick_id : u32,
}

impl Item {
    fn new_error() -> Item {
        let texture = AssetManager::get_asset_id("error_fallbacks/texture.png");
        let sprite = Sprite::new(texture);
        Item { 
            id : ItemId(0), 
            sprite,
            positions : HashMap::new(),
            from_last_tick : 0.0,
            last_tick_id : std::u32::MAX
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
                    positions : HashMap::new(),
                    from_last_tick : 0.0,
                    last_tick_id : std::u32::MAX
                };
                
                new_item
            }
            _ => { 
                log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure"); 
                Self::new_error()
            }
        }
    }

    pub fn set_position_in_tick(&mut self, position : Vec2, tick_id : u32) {
        if self.positions.contains_key(&TickId(tick_id)) {
            *self.positions.get_mut(&TickId(tick_id)).unwrap() = position;
        } else {
            self.positions.insert(TickId(tick_id), position);
        }
    }
}

impl GameEntity for Item {
    // TODO : OPTIMIZE!!!
    fn update(&mut self, delta_time : f32) {
        self.from_last_tick += delta_time;

        let mut remove_keys = Vec::new();
        for key in self.positions.keys() {
            if key.0 + 5 < self.last_tick_id {
                remove_keys.push(*key);
            }
        }
        for key in remove_keys { self.positions.remove(&key); }

        let mut next_interpolate_tick = None;
        for tick in self.positions.keys() {
            if tick.0 > self.last_tick_id {
                if next_interpolate_tick.is_none() {
                    next_interpolate_tick = Some(tick);
                } else {
                    if tick.0 < next_interpolate_tick.unwrap().0 {
                        next_interpolate_tick = Some(tick);
                    }
                }
            }
        }

        let mut prev_interpolate_tick = None;
        for tick in self.positions.keys() {
            if tick.0 <= self.last_tick_id {
                if prev_interpolate_tick.is_none() {
                    prev_interpolate_tick = Some(tick);
                } else {
                    if tick.0 > prev_interpolate_tick.unwrap().0 {
                        prev_interpolate_tick = Some(tick);
                    }
                }
            }
        }

        if next_interpolate_tick.is_none() {
            if prev_interpolate_tick.is_none() {
                self.sprite.position = Vec2::zero();
                log::error!("Item {} failed to update it's position : no set_position_in_tick() calls before update", self.id.0);
            } else {
                self.sprite.position = *self.positions.get(prev_interpolate_tick.unwrap()).unwrap();
            }
        } else {
            if prev_interpolate_tick.is_none() {
                self.sprite.position = *self.positions.get(next_interpolate_tick.unwrap()).unwrap();
            } else {
                let prev_interpolate_tick = prev_interpolate_tick.unwrap();
                let next_interpolate_tick = next_interpolate_tick.unwrap();

                let tick_interval = next_interpolate_tick.0 - prev_interpolate_tick.0;
                let tick_interval = tick_interval as f32 * crate::game::TICK_PERIOD;
                let curr_time = self.last_tick_id - prev_interpolate_tick.0;
                let curr_time = curr_time as f32 * crate::game::TICK_PERIOD + self.from_last_tick;
                let t = curr_time / tick_interval;
                let prev_pos = *self.positions.get(&prev_interpolate_tick).unwrap();
                let next_pos = *self.positions.get(&next_interpolate_tick).unwrap();
                self.sprite.position = prev_pos + (next_pos - prev_pos) * t;
            }
        }
    }

    fn tick(&mut self, tick_id : u32) {
        self.from_last_tick = 0.0;
        self.last_tick_id = tick_id;
    }

    fn render(&mut self, renderer : &mut Renderer) {
        renderer.queue_render_sprite(&self.sprite);
    }
}

impl Clone for Item { 
    fn clone(&self) -> Item {
        Item { 
            id : self.id, 
            sprite : self.sprite.clone(),
            positions : self.positions.clone(),
            from_last_tick : self.from_last_tick,
            last_tick_id : self.last_tick_id
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