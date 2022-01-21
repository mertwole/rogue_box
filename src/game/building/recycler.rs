use std::collections::HashMap;

use super::*;

use crate::game::resource::item::*;
use crate::game::renderer::{Renderer, Sprite};
use crate::game::game_entity::GameEntity;
use crate::game::building::transport_belt::TransportedItem;
use crate::common::asset_manager::{AssetId, AssetManager};
use crate::common::json_reader::JsonReader;
use crate::game::location::surface::*;

pub struct Recycler {
    name : String,
    texture : AssetId,

    period : u32,
    from_last_production : u32,
    can_produce : bool,

    item_input : HashMap<ItemId, u32>,
    item_output : HashMap<ItemId, u32>,

    item_input_buf : HashMap<ItemId, u32>,
    item_output_buf : HashMap<ItemId, u32>,

    item_prototypes : HashMap<ItemId, Item>
}

enum PlacementRequirement {
    RequireSurfaces(Vec<SurfaceId>)
}

impl Recycler {
    pub fn from_json_object(obj : &serde_json::Value) -> Recycler {
        let mut error = false;

        let name = JsonReader::read_string(obj, "name", &mut error);

        let tex_path = JsonReader::read_string(obj, "texture", &mut error);
        let texture = AssetManager::get_asset_id(&tex_path);

        let period = JsonReader::read_i32(obj, "period", &mut error) as u32;
        
        let items = JsonReader::read_obj(obj, "items", &mut error);

        let item_input_vec = JsonReader::read_vec(&items, "input", &mut error);
        let item_input_vec : Vec<(String, u32)> = item_input_vec.iter()
        .map(|item| { 
            let name = JsonReader::read_string(item, "item", &mut error);
            let amount = JsonReader::read_i32(item, "amount", &mut error) as u32;
            (name, amount)
        })
        .collect();

        let mut item_input = HashMap::new();
        let mut item_input_buf = HashMap::new();
        for (item, amount) in item_input_vec {
            let id = ItemFactory::get_item_id_by_name(&item);
            item_input.insert(id, amount);
            item_input_buf.insert(id, 0);
        }

        let item_output_vec = JsonReader::read_vec(&items, "output", &mut error);
        let item_output_vec : Vec<(String, u32)> = item_output_vec.iter()
        .map(|item| { 
            let name = JsonReader::read_string(item, "item", &mut error);
            let amount = JsonReader::read_i32(item, "amount", &mut error) as u32;
            (name, amount)
        })
        .collect();

        let mut item_output = HashMap::new();
        let mut item_output_buf = HashMap::new();
        for (item, amount) in item_output_vec {
            let id = ItemFactory::get_item_id_by_name(&item);
            item_output.insert(id, amount);
            item_output_buf.insert(id, 0);
        }

        if error {
            log::error!("Failed to parse Recycler from json ({})", 
            if name.is_empty() { "error loading name" } else { &name });
        } else {
            log::info!("Recycler succesfully loaded({})", name);
        }

        Recycler {
            name,
            texture,

            period,
            from_last_production : 0,
            can_produce : false,

            item_input,
            item_output,

            item_input_buf,
            item_output_buf,

            item_prototypes : HashMap::new()
        }
    }

    pub fn init_items(&mut self, item_factory : &ItemFactory) {
        let item_ids = self.item_input.keys().chain(self.item_output.keys());
        for &item_id in item_ids {
            if !self.item_prototypes.contains_key(&item_id) {
                self.item_prototypes.insert(item_id, item_factory.create_item(item_id));
            }
        }
    }

    fn drain_output(&mut self) {
        for id in self.item_output.keys() {
            *self.item_output_buf.get_mut(&id).unwrap() = 0;
        }
    }
}

impl GameEntity for Recycler {
    fn update(&mut self, parameters : &UpdateParameters) {

    }

    fn tick(&mut self, tick_id : u32) {
        if self.can_produce {
            self.from_last_production += 1;
            if self.from_last_production >= self.period {
                for (id, &amount) in &self.item_output {
                    *self.item_output_buf.get_mut(&id).unwrap() = amount;
                }
                self.can_produce = false;
            }
        }
        else {
            let mut input_buffer_full = true;
            for (item, &amount) in &self.item_input_buf {
                if amount < *self.item_input.get(item).unwrap() {
                    input_buffer_full = false;
                    break;
                }
            }
            if input_buffer_full {
                for amount in self.item_input_buf.values_mut() { *amount = 0; }
                self.can_produce = true;
                self.from_last_production = 0;
            }
        } 
    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        let mut sprite = Sprite::new(self.texture);
        renderer.queue_render_sprite(sprite, transform);
    }
}

impl BuildingClone for Recycler {
    fn clone_box(&self) -> Box<dyn Building> {
        let mut item_input_buf = self.item_input_buf.clone();
        for val in item_input_buf.values_mut() { *val = 0; }
        let mut item_output_buf = self.item_output_buf.clone();
        for val in item_output_buf.values_mut() { *val = 0; }

        Box::from(
            Recycler {
                name : self.name.clone(),
                texture : self.texture,

                period : self.period,
                from_last_production : 0,
                can_produce : false,

                item_input : self.item_input.clone(),
                item_output : self.item_output.clone(),

                item_input_buf,
                item_output_buf,

                item_prototypes : self.item_prototypes.clone()
            }
        )
    }   
}

impl Building for Recycler {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl MessageSender for Recycler {
    fn pull_messages(&mut self, tick_id : u32) -> Vec<Message> {
        let mut messages = Vec::new();
        for item_id in self.item_output.keys() {
            let item_count = *self.item_output_buf.get(item_id).unwrap();
            let item_prototype = self.item_prototypes.get(item_id).unwrap();            
            for _ in  0..item_count {
                messages.push(Message {
                    id : messages.len() as u32,
                    sender : MessageExchangeActor::NotComputedYet,
                    receiver : MessageExchangeActor::NotComputedYet,
                    target : Target::Broadcast,
                    tick_id,
                    body : MessageBody::PushItem(TransportedItem::new(item_prototype.clone()))
                });
            }
        }

        self.drain_output();

        messages
    }

    fn message_send_result(&mut self, result : MessageSendResult) { 
        match result.message {
            Some(message) => { 
                match &message.body {
                    MessageBody::PushItem(item) => {
                        *self.item_output_buf.get_mut(&item.get_id()).unwrap() += 1;
                    }
                    _ => { }
                }
            }
            None => { return; }
        }
    }
}

impl MessageReceiver for Recycler {
    fn try_push_message(&mut self, message : Message) -> Option<Message> {
        Some(message)
    }
}