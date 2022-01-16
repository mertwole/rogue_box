use std::collections::HashMap;
use std::iter;

use super::*;
use crate::game::game_entity::*;
use crate::game::resource::item::Item;
use crate::common::direction::Direction;
use crate::common::json_reader::JsonReader;
use crate::common::math::IVec2;

pub struct TransportedItem {
    item : Item,
    // Item can be moved by transport belts only once per tick.
    last_tick_moved : u32
}

impl TransportedItem {
    pub fn new(item : Item) -> TransportedItem {
        TransportedItem {
            item,
            last_tick_moved : 0
        }
    } 
}

pub struct TransportBelt { 
    name : String,

    inputs : Vec<Direction>,
    output : Direction,
    // For input buffers 0th element in vec is the edge cell
    // then to center(exclusive) by upscending.
    //
    // For output buffer 0th element is center cell
    // then to the edge(exclusive) by upscending.
    pub/*DEBUG*/ item_buffers : HashMap<Direction, Vec<Option<TransportedItem>>>,
    // Item count on the one side of the belt
    // so max capacity of belt = item_count * 4.
    item_count : u32,

    pub /*DEBUG*/ position : IVec2
}

impl TransportBelt {
    pub fn from_json_object(obj : &serde_json::Value) -> TransportBelt {
        let mut error = false;

        let name = JsonReader::read_string(obj, "name", &mut error);
        let item_count = JsonReader::read_i32(obj, "item_count", &mut error) as u32;

        if error {
            log::error!("Failed to parse TransportBelt from json ({})", 
            if name.is_empty() { "error loading name" } else { &name });
        } else {
            log::info!("TransportBelt succesfully loaded({})", name);
        }

        TransportBelt {
            name,
            inputs : Vec::new(),
            output : Direction::None,
            item_buffers : HashMap::new(),
            item_count,
            position : IVec2::zero()
        }    
    }

    pub fn set_config(&mut self, inputs : Vec<Direction>, output : Direction) {
        self.inputs = inputs;
        self.output = output;

        for &dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let mut buffer = Vec::with_capacity(self.item_count as usize);
            for _ in 0..self.item_count { buffer.push(None); }
            self.item_buffers.insert(dir, buffer);
        }
    }

    fn move_buffer_items(&mut self, direction : Direction, tick_id : u32) {
        let buffer = self.item_buffers.get_mut(&direction).unwrap();
        for i in (0..self.item_count as usize - 1).rev() {
            if buffer[i].is_some() {
                let last_tick_moved = buffer.get(i).unwrap().as_ref().unwrap().last_tick_moved;
                if buffer[i + 1].is_none() && last_tick_moved != tick_id {
                    let mut item = buffer[i].take();
                    item.as_mut().unwrap().last_tick_moved = tick_id;
                    buffer[i + 1] = item;
                }
            }
        }
    }

    fn move_items(&mut self, tick_id : u32) {
        // Move output buffer.
        self.move_buffer_items(self.output, tick_id);
        // Move last items of inputs to center.
        let out_buffer = self.item_buffers.get_mut(&self.output).unwrap();
        let mut push_to_center = None;
        if out_buffer[0].is_none() {
            for dir in &self.inputs {
                let buffer = self.item_buffers.get_mut(dir).unwrap();
                if buffer.last().unwrap().is_some() {
                    push_to_center = buffer[self.item_count as usize - 1].take();
                    push_to_center.as_mut().unwrap().last_tick_moved = tick_id;
                    break;
                }
            }
        }

        if push_to_center.is_some() {
            let out_buffer = self.item_buffers.get_mut(&self.output).unwrap();
            out_buffer[0].replace(push_to_center.unwrap());
        }
        // Move input buffers.
        for &dir in &self.inputs.clone() {
            self.move_buffer_items(dir, tick_id);
        }
    }
}

impl GameEntity for TransportBelt {
    fn update(&mut self, delta_time : f32) {
        for dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let buffer = self.item_buffers.get_mut(dir).unwrap();
            for item in buffer.iter_mut().enumerate() {
                match item {
                    (item_id, Some(item)) => {
                        item.item.update(delta_time);
                        // DEBUG UPDATE POSITION.
                        let dir_vec = dir.to_ivec2().to_vec2() / (2.0 * self.item_count as f32);
                        let rel_pos = dir_vec * if self.output == *dir { 
                            item_id as f32 
                        } else { 
                            self.item_count as f32 - item_id as f32 
                        };
                        item.item.set_position(self.position.to_vec2() + rel_pos);
                    }
                    _ => { }
                }
            }
        }
    }

    fn tick(&mut self, tick_id : u32) {
        self.move_items(tick_id);

        for dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let buffer = self.item_buffers.get_mut(dir).unwrap();
            for item in buffer {
                match item {
                    Some(item) => {
                        item.item.tick(tick_id);
                    }
                    None => { }
                }
            }
        }
    }

    fn render(&mut self, renderer : &mut Renderer) {
        for dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let buffer = self.item_buffers.get_mut(dir).unwrap();
            for item in buffer {
                match item {
                    Some(item) => {
                        item.item.render(renderer);
                    }
                    None => { }
                }
            }
        }
    }
}

impl BuildingClone for TransportBelt {
    fn clone_box(&self) -> Box<dyn Building> {
        Box::from(TransportBelt { 
            name : self.name.clone(),
            inputs : Vec::new(),
            output : Direction::None,
            item_buffers : HashMap::new(),
            item_count : self.item_count,
            position : IVec2::zero()
        })
    }
}

impl Building for TransportBelt {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl MessageSender for TransportBelt {
    fn pull_messages(&mut self) -> Vec<Message> {
        let output_buf = self.item_buffers.get_mut(&self.output).unwrap();
        if output_buf.last().unwrap().is_some() {
            let item = output_buf[self.item_count as usize - 1].take();
            vec![ 
                Message { 
                    sender : self.position, 
                    receiver : self.position + self.output.to_ivec2(), 
                    tick_id : 0,
                    body : MessageBody::PushItem(item.unwrap())
                }
            ]
        } else { vec![] }
    }

    fn push_back_message(&mut self, message : Message) {
        match message.body {
            MessageBody::PushItem(item) => {
                let out_buf = self.item_buffers.get_mut(&self.output).unwrap();
                out_buf[self.item_count as usize - 1] = Some(item);
            }
            _ => { }
        }
    }
}

impl MessageReceiver for TransportBelt {
    fn try_push_message(&mut self, message : Message) -> Option<Message> {
        match &message.body {
            MessageBody::PushItem(item) => { 
                if item.last_tick_moved == message.tick_id {
                    return Some(message);
                }

                let direction = Direction::from_ivec2(message.sender - message.receiver);
                if self.inputs.contains(&direction) {
                    let input = self.item_buffers.get_mut(&direction).unwrap();
                    if input[0].is_none() { 
                        match message.body {
                            MessageBody::PushItem(mut item) => {
                                item.last_tick_moved = message.tick_id;
                                input[0] = Some(item); 
                                return None;
                            }
                            _ => { }
                        }
                    }
                }
                Some(message)
            }
            _ => { Some(message) }
        }
    }
}