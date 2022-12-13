use std::collections::HashMap;
use std::iter;

use super::item::*;
use crate::game::{
    common::{direction::Direction, json_reader::JsonReader, math::Vec2},
    game_entity::GameEntity,
    gui::with_gui::*,
    location::field::building::{Building, BuildingClone},
    message::*,
    Renderer, SpriteTransform, UpdateParameters,
};

pub struct TransportBelt {
    name: String,

    inputs: Vec<Direction>,
    output: Direction,
    // For input buffers 0th element in vec is the edge cell
    // then to center(exclusive) by upscending.
    //
    // For output buffer 0th element is center cell
    // then to the edge(exclusive) by upscending.
    pub item_buffers: HashMap<Direction, Vec<Option<TransportedItem>>>,
    // Item count on the one side of the belt
    // so max capacity of belt = item_count * 4.
    item_count: u32,
}

impl TransportBelt {
    pub fn from_json_object(obj: &serde_json::Value) -> TransportBelt {
        let mut error = false;

        let name = JsonReader::read_string(obj, "name", &mut error);
        let item_count = JsonReader::read_i32(obj, "item_count", &mut error) as u32;

        if error {
            log::error!(
                "Failed to parse TransportBelt from json ({})",
                if name.is_empty() {
                    "error loading name"
                } else {
                    &name
                }
            );
        } else {
            log::info!("TransportBelt succesfully loaded({})", name);
        }

        TransportBelt {
            name,
            inputs: Vec::new(),
            output: Direction::None,
            item_buffers: HashMap::new(),
            item_count,
        }
    }

    pub fn set_config(&mut self, inputs: Vec<Direction>, output: Direction) {
        self.inputs = inputs;
        self.output = output;

        for &dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let mut buffer = Vec::with_capacity(self.item_count as usize);
            for _ in 0..self.item_count {
                buffer.push(None);
            }
            self.item_buffers.insert(dir, buffer);
        }
    }

    fn compute_item_position(&self, direction: Direction, vec_id: i32) -> Vec2 {
        let dir_vec = direction.to_ivec2().to_vec2() / (2.0 * self.item_count as f32);
        dir_vec
            * if self.output == direction {
                vec_id as f32
            } else {
                self.item_count as f32 - vec_id as f32
            }
    }

    fn move_buffer_items(&mut self, direction: Direction, tick_id: u32) {
        for i in (0..self.item_count as usize - 1).rev() {
            let move_from = self.compute_item_position(direction, i as i32);
            let move_to = self.compute_item_position(direction, i as i32 + 1);

            let buffer = self.item_buffers.get_mut(&direction).unwrap();
            if buffer[i].is_some() {
                let last_tick_moved = buffer[i].as_ref().unwrap().last_tick_moved;
                if buffer[i + 1].is_none() && last_tick_moved != tick_id {
                    let mut item = buffer[i].take();
                    item.as_mut().unwrap().last_tick_moved = tick_id;
                    item.as_mut()
                        .unwrap()
                        .set_movement(move_from, move_to, tick_id);
                    buffer[i + 1] = item;
                }
            }
        }
    }

    fn move_last_input_buffer_items(&mut self, tick_id: u32) {
        let out_buffer = self.item_buffers.get_mut(&self.output).unwrap();
        let mut push_to_center = None;
        let mut move_from = Vec2::zero();
        if out_buffer[0].is_none() {
            for dir in &self.inputs {
                let buffer = self.item_buffers.get_mut(dir).unwrap();
                if (*buffer.last().unwrap()).is_some() {
                    let last_tick_moved = buffer.last().unwrap().as_ref().unwrap().last_tick_moved;
                    if last_tick_moved != tick_id {
                        push_to_center = buffer[self.item_count as usize - 1].take();
                        push_to_center.as_mut().unwrap().last_tick_moved = tick_id;
                        move_from = self.compute_item_position(*dir, self.item_count as i32 - 1);
                        break;
                    }
                }
            }
        }

        if push_to_center.is_some() {
            let move_to = self.compute_item_position(self.output, 0);
            let out_buffer = self.item_buffers.get_mut(&self.output).unwrap();
            (*out_buffer)[0] = push_to_center;
            (*out_buffer)[0]
                .as_mut()
                .unwrap()
                .set_movement(move_from, move_to, tick_id);
        }
    }

    fn move_items(&mut self, tick_id: u32) {
        self.move_buffer_items(self.output, tick_id);
        self.move_last_input_buffer_items(tick_id);
        for &dir in &self.inputs.clone() {
            self.move_buffer_items(dir, tick_id);
        }
    }

    fn try_push_item(
        &mut self,
        mut item: TransportedItem,
        direction: Direction,
        tick_id: u32,
    ) -> Option<TransportedItem> {
        if !self.inputs.contains(&direction) {
            return Some(item);
        }

        let move_from = self.compute_item_position(direction, -1);
        let move_to = self.compute_item_position(direction, 0);

        let input = self.item_buffers.get_mut(&direction).unwrap();
        if input[0].is_some() {
            return Some(item);
        }
        if item.last_tick_moved == tick_id {
            return Some(item);
        }

        item.set_movement(move_from, move_to, tick_id);
        item.last_tick_moved = tick_id;
        input[0].replace(item);
        None
    }

    fn pull_item(&mut self, tick_id: u32) -> Option<TransportedItem> {
        let output_buf = self.item_buffers.get_mut(&self.output).unwrap();
        let item = output_buf.last_mut().unwrap().as_mut()?;
        if item.last_tick_moved == tick_id {
            return None;
        }
        let mut item = output_buf.last_mut().unwrap().take().unwrap();
        let move_from = self.compute_item_position(self.output, self.item_count as i32 - 1);
        let move_to = self.compute_item_position(self.output, self.item_count as i32);
        item.set_movement(move_from, move_to, tick_id);
        Some(item)
    }

    fn pull_item_failed(&mut self, mut item: TransportedItem, tick_id: u32) {
        let pos = self.compute_item_position(self.output, self.item_count as i32 - 1);
        item.set_movement(pos, pos, tick_id);

        let output_buf = self.item_buffers.get_mut(&self.output).unwrap();
        output_buf.last_mut().unwrap().replace(item);
    }
}

impl GameEntity for TransportBelt {
    fn update(&mut self, parameters: &UpdateParameters) {
        for dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let buffer = self.item_buffers.get_mut(dir).unwrap();
            for item in buffer {
                match item {
                    Some(item) => {
                        item.update(parameters);
                    }
                    None => {}
                }
            }
        }
    }

    fn tick(&mut self, tick_id: u32) {
        self.move_items(tick_id);

        for dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let buffer = self.item_buffers.get_mut(dir).unwrap();
            for item in buffer {
                match item {
                    Some(item) => {
                        item.tick(tick_id);
                    }
                    None => {}
                }
            }
        }
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        for dir in self.inputs.iter().chain(iter::once(&self.output)) {
            let buffer = self.item_buffers.get_mut(dir).unwrap();
            for item in buffer {
                match item {
                    Some(item) => {
                        item.render(renderer, transform.clone());
                    }
                    None => {}
                }
            }
        }
    }
}

impl WithGui for TransportBelt {
    fn render_gui(&mut self, params: &mut GuiRenderParams) {}
}

impl BuildingClone for TransportBelt {
    fn clone_box(&self) -> Box<dyn Building> {
        Box::from(TransportBelt {
            name: self.name.clone(),
            inputs: Vec::new(),
            output: Direction::None,
            item_buffers: HashMap::new(),
            item_count: self.item_count,
        })
    }
}

impl Building for TransportBelt {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl MessageSender for TransportBelt {
    fn pull_messages(&mut self, tick_id: u32) -> Vec<Message> {
        let pulled_item = self.pull_item(tick_id);

        match pulled_item {
            Some(item) => {
                vec![Message::FieldMessage(field_message::Message {
                    id: 0,
                    sender: field_message::MessageExchangeActor::default(),
                    receiver: field_message::MessageExchangeActor::default(),
                    target: field_message::Target::Directions(vec![self.output]),
                    tick_id,
                    refund: false,
                    body: field_message::MessageBody::PushItem(item),
                })]
            }
            None => {
                vec![]
            }
        }
    }
}

impl MessageReceiver for TransportBelt {
    fn try_push_message(&mut self, mut message: Message) -> Option<Message> {
        match message {
            Message::FieldMessage(ref mut msg) => match &msg.body {
                field_message::MessageBody::PushItem(item) => {
                    let item = item.clone();
                    if msg.refund {
                        self.pull_item_failed(item, msg.tick_id);
                        None
                    } else {
                        if item.last_tick_moved == msg.tick_id {
                            msg.body = field_message::MessageBody::PushItem(item);
                            return Some(message);
                        }
                        let direction = Direction::from_ivec2(
                            msg.sender.get_position() - msg.receiver.get_position(),
                        );
                        let push_result = self.try_push_item(item, direction, msg.tick_id);
                        match push_result {
                            Some(item) => {
                                msg.body = field_message::MessageBody::PushItem(item);
                                Some(message)
                            }
                            None => None,
                        }
                    }
                }
                _ => Some(message),
            },
            _ => Some(message),
        }
    }
}
