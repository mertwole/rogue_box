use std::collections::HashMap;

use super::*;

use crate::game::hub::item::*;
use crate::game::renderer::{Renderer, Sprite};
use crate::game::game_entity::GameEntity;
use crate::game::hub::item::TransportedItem;
use crate::game::common::asset_manager::{AssetId, AssetManager};
use crate::game::common::json_reader::JsonReader;
use crate::game::hub::electric_port::*;

pub struct Recycler {
    name : String,
    texture : AssetId,

    period : u32,
    from_last_production : u32,
    can_produce : bool,

    produces_electricity : u32,
    can_produce_electricity : bool,
    // Items.
    item_input : HashMap<ItemId, u32>,
    item_output : HashMap<ItemId, u32>,

    item_input_buf : HashMap<ItemId, u32>,
    item_output_buf : HashMap<ItemId, u32>,

    item_prototypes : HashMap<ItemId, Item>,
    // Electricity.
    electric_ports : Vec<Box<dyn ElectricPort>>
}

impl Recycler {
    fn common_data_from_json_object(&mut self, obj : &serde_json::Value, error : &mut bool) {
        self.name = JsonReader::read_string(obj, "name", error);

        let tex_path = JsonReader::read_string(obj, "texture", error);
        self.texture = AssetManager::get_asset_id(&tex_path);

        self.period = JsonReader::read_i32(obj, "period", error) as u32;
    } 

    fn item_data_from_json_object(&mut self, obj : &serde_json::Value, error : &mut bool) {
        let items = JsonReader::read_obj(obj, "items", error);

        let item_input_vec = JsonReader::read_vec(&items, "input", error);
        let item_input_vec : Vec<(String, u32)> = item_input_vec.iter()
        .map(|item| { 
            let name = JsonReader::read_string(item, "item", error);
            let amount = JsonReader::read_i32(item, "amount", error) as u32;
            (name, amount)
        })
        .collect();

        self.item_input = HashMap::new();
        self.item_input_buf = HashMap::new();
        for (item, amount) in item_input_vec {
            let id = ItemFactory::get_item_id_by_name(&item);
            self.item_input.insert(id, amount);
            self.item_input_buf.insert(id, 0);
        }

        let item_output_vec = JsonReader::read_vec(&items, "output", error);
        let item_output_vec : Vec<(String, u32)> = item_output_vec.iter()
        .map(|item| { 
            let name = JsonReader::read_string(item, "item", error);
            let amount = JsonReader::read_i32(item, "amount", error) as u32;
            (name, amount)
        })
        .collect();

        self.item_output = HashMap::new();
        self.item_output_buf = HashMap::new();
        for (item, amount) in item_output_vec {
            let id = ItemFactory::get_item_id_by_name(&item);
            self.item_output.insert(id, amount);
            self.item_output_buf.insert(id, 0);
        }
    }

    fn electricity_data_from_json_object(&mut self, obj : &serde_json::Value, error : &mut bool) {
        let electric_port_objs = JsonReader::read_vec(obj, "electric_ports", error);
        self.electric_ports = Vec::new();

        for port_obj in electric_port_objs {
            let voltage = JsonReader::read_i32(&port_obj, "voltage", error) as u32;
            let voltage = Voltage::new(voltage);
            let energy = JsonReader::read_i32(&port_obj, "energy", error) as u32;
            let energy = WattTick::new(energy);

            let mode = JsonReader::read_string(&port_obj, "mode", error);
            let port_id = PortId::new(self.electric_ports.len() as u32);
            let port : Box<dyn ElectricPort> = match &*mode {
                "in" => { Box::from(ElectricInput::new(voltage, energy, port_id)) }
                "out" => { Box::from(ElectricOutput::new(voltage, energy, port_id)) }
                _ => { *error = true; return; }
            };

            self.electric_ports.push(port);
        }
    }

    pub fn from_json_object(obj : &serde_json::Value) -> Recycler {
        let mut recycler = Recycler {
            name : String::new(),
            texture : AssetId::null(),

            period : 0,
            from_last_production : 0,
            can_produce : false,
            produces_electricity : 0,
            can_produce_electricity : false,

            item_input : HashMap::new(),
            item_output : HashMap::new(),
            item_input_buf : HashMap::new(),
            item_output_buf : HashMap::new(),
            item_prototypes : HashMap::new(),

            electric_ports : Vec::new()
        };

        let mut error = false;

        recycler.common_data_from_json_object(obj, &mut error);
        recycler.item_data_from_json_object(obj, &mut error);
        recycler.electricity_data_from_json_object(obj, &mut error);

        if error {
            log::error!("Failed to parse Recycler from json ({})", 
            if recycler.name.is_empty() { "error loading name" } else { &recycler.name });
        } else {
            log::info!("Recycler succesfully loaded({})", recycler.name);
        }

        recycler
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
        if self.can_produce_electricity {
            
            for port in &mut self.electric_ports {
                let port = port.as_mut();
                match port.as_output_mut() {
                    Some(out) => { out.fill(); }
                    None => { }
                }
            }

            self.produces_electricity += 1;
            if self.produces_electricity >= self.period {
                self.can_produce_electricity = false;
            }
        }

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
            let mut can_take_resources = true;
            for (item, &amount) in &self.item_input_buf {
                if amount < *self.item_input.get(item).unwrap() {
                    can_take_resources = false;
                    break;
                }
            }

            for port in &self.electric_ports {
                let port = port.as_ref();
                match port.as_input() {
                    Some(inp) => { 
                        if !inp.is_full() {
                            can_take_resources = false;
                            break;
                        }
                    }
                    None => { }
                }
            }

            if can_take_resources {
                for amount in self.item_input_buf.values_mut() { *amount = 0; }

                for port in &mut self.electric_ports {
                    let port = port.as_mut();
                    match port.as_input_mut() {
                        Some(inp) => { inp.drain(); }
                        None => { }
                    }
                }

                self.can_produce = true;
                self.from_last_production = 0;
                self.can_produce_electricity = true;
                self.produces_electricity = 0;
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

        let mut electric_ports = Vec::new();
        for port in &self.electric_ports { electric_ports.push((*port).clone_box()); }

        Box::from(
            Recycler {
                name : self.name.clone(),
                texture : self.texture,

                period : self.period,
                from_last_production : 0,
                can_produce : false,
                produces_electricity : 0,
                can_produce_electricity : false,

                item_input : self.item_input.clone(),
                item_output : self.item_output.clone(),

                item_input_buf,
                item_output_buf,

                item_prototypes : self.item_prototypes.clone(),

                electric_ports
            }
        )
    }   
}

impl Building for Recycler {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_electric_ports_mut(&mut self) -> Vec<&mut dyn ElectricPort> { 
        vec![]
    }

    fn get_electric_ports(&self) -> Vec<&dyn ElectricPort> { 
        vec![] 
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
                    sender : MessageExchangeActor::new(),
                    receiver : MessageExchangeActor::new(),
                    target : Target::BroadcastNeighbors,
                    tick_id,
                    body : MessageBody::PushItem(TransportedItem::new(item_prototype.clone()))
                });
            }
        }

        self.drain_output();

        for port in &mut self.electric_ports {
            match port.as_mut().as_output_mut() {
                Some(out) => { messages.append(&mut out.pull_messages(tick_id)); }
                None => { }
            }
        }

        messages
    }

    fn message_send_result(&mut self, result : MessageSendResult) { 
        match &result.message {
            Some(message) => { 
                match &message.body {
                    MessageBody::PushItem(item) => {
                        *self.item_output_buf.get_mut(&item.get_id()).unwrap() += 1;
                    }
                    MessageBody::SendElectricity(_) => { 
                        let sender_port = message.sender.get_electric_port();
                        for port in &mut self.electric_ports {
                            if port.get_id() == sender_port {
                                match port.as_mut().as_output_mut() {
                                    Some(out) => { 
                                        out.message_send_result(result);
                                        return;
                                    }
                                    None => { }
                                }
                            }
                        }
                    }
                }
            }
            None => { return; }
        }
    }
}

impl MessageReceiver for Recycler {
    fn try_push_message(&mut self, mut message : Message) -> Option<Message> {
        match &message.body {
            MessageBody::PushItem(item) => { 
                let item_id = item.get_id();
                if self.item_input.contains_key(&item_id) {
                    let inp_buf = self.item_input_buf.get_mut(&item_id).unwrap();
                    if *inp_buf < *self.item_input.get(&item_id).unwrap() {
                        *inp_buf += 1;
                        return None;
                    }
                }
                Some(message)
            }
            _ => {
                let receiver_port = message.receiver.get_electric_port();
                for port in &mut self.electric_ports {
                    if port.get_id() != receiver_port { continue; }
                    match port.as_input_mut() {
                        Some(inp) => { 
                            message = match inp.try_push_message(message) {
                                Some(back) => { back }
                                None => { return None; }
                            }
                        }
                        None => { }
                    }
                }

                Some(message)
            }
        }
    }
}