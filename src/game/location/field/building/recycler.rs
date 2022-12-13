use std::collections::HashMap;

use super::*;

use crate::game::{
    common::{
        asset_manager::{AssetId, AssetManager},
        direction::Direction,
        json_reader::JsonReader,
    },
    game_entity::GameEntity,
    location::field::building::item::{Item, ItemFactory, ItemId, TransportedItem},
    message::*,
    renderer::{Renderer, Sprite},
};

pub struct Recycler {
    name: String,
    texture: AssetId,

    period: u32,
    from_last_production: u32,
    can_produce: bool,

    // Items.
    item_input: HashMap<ItemId, u32>,
    item_output: HashMap<ItemId, u32>,

    item_input_buf: HashMap<ItemId, u32>,
    item_output_buf: HashMap<ItemId, u32>,

    item_prototypes: HashMap<ItemId, Item>,
}

impl Recycler {
    fn common_data_from_json_object(&mut self, obj: &serde_json::Value, error: &mut bool) {
        self.name = JsonReader::read_string(obj, "name", error);

        let tex_path = JsonReader::read_string(obj, "texture", error);
        self.texture = AssetManager::get_asset_id(&tex_path);

        self.period = JsonReader::read_i32(obj, "period", error) as u32;
    }

    fn item_data_from_json_object(&mut self, obj: &serde_json::Value, error: &mut bool) {
        let items = JsonReader::read_obj(obj, "items", error);

        let item_input_vec = JsonReader::read_vec(&items, "input", error);
        let item_input_vec: Vec<(String, u32)> = item_input_vec
            .iter()
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
        let item_output_vec: Vec<(String, u32)> = item_output_vec
            .iter()
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

    pub fn from_json_object(obj: &serde_json::Value) -> Recycler {
        let mut recycler = Recycler {
            name: String::new(),
            texture: AssetId::null(),

            period: 0,
            from_last_production: 0,
            can_produce: false,

            item_input: HashMap::new(),
            item_output: HashMap::new(),
            item_input_buf: HashMap::new(),
            item_output_buf: HashMap::new(),
            item_prototypes: HashMap::new(),
        };

        let mut error = false;

        recycler.common_data_from_json_object(obj, &mut error);
        recycler.item_data_from_json_object(obj, &mut error);

        if error {
            log::error!(
                "Failed to parse Recycler from json ({})",
                if recycler.name.is_empty() {
                    "error loading name"
                } else {
                    &recycler.name
                }
            );
        } else {
            log::info!("Recycler succesfully loaded({})", recycler.name);
        }

        recycler
    }

    pub fn init_items(&mut self, item_factory: &ItemFactory) {
        let item_ids = self.item_input.keys().chain(self.item_output.keys());
        for &item_id in item_ids {
            self.item_prototypes
                .entry(item_id)
                .or_insert_with(|| item_factory.create_item(item_id));
        }
    }

    fn pull_item_messages(&mut self, tick_id: u32) -> Vec<Message> {
        let mut messages = Vec::new();
        for item_id in self.item_output.keys() {
            let item_count = *self.item_output_buf.get(item_id).unwrap();
            let item_prototype = self.item_prototypes.get(item_id).unwrap();

            for _ in 0..item_count {
                messages.push(Message::FieldMessage(field_message::Message {
                    id: messages.len() as u32,
                    sender: field_message::MessageExchangeActor::default(),
                    receiver: field_message::MessageExchangeActor::default(),
                    target: field_message::Target::Directions(vec![
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                    ]),
                    tick_id,
                    refund: false,
                    body: field_message::MessageBody::PushItem(TransportedItem::new(
                        item_prototype.clone(),
                    )),
                }));
            }
        }

        for id in self.item_output.keys() {
            *self.item_output_buf.get_mut(id).unwrap() = 0;
        }

        messages
    }
}

impl GameEntity for Recycler {
    fn update(&mut self, parameters: &UpdateParameters) {}

    fn tick(&mut self, tick_id: u32) {
        if self.can_produce {
            self.from_last_production += 1;
            if self.from_last_production >= self.period {
                for (id, &amount) in &self.item_output {
                    *self.item_output_buf.get_mut(id).unwrap() = amount;
                }
                self.can_produce = false;
            }
        }

        if !self.can_produce {
            let mut can_take_resources = true;
            for (item, &amount) in &self.item_input_buf {
                if amount < *self.item_input.get(item).unwrap() {
                    can_take_resources = false;
                    break;
                }
            }

            if can_take_resources {
                for amount in self.item_input_buf.values_mut() {
                    *amount = 0;
                }

                self.can_produce = true;
                self.from_last_production = 0;
            }
        }
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        let sprite = Sprite::new(self.texture);
        renderer.queue_render_sprite(sprite, transform);
    }
}

impl WithGui for Recycler {
    fn render_gui(&mut self, params: &mut GuiRenderParams) {}
}

impl BuildingClone for Recycler {
    fn clone_box(&self) -> Box<dyn Building> {
        let mut item_input_buf = self.item_input_buf.clone();
        for val in item_input_buf.values_mut() {
            *val = 0;
        }
        let mut item_output_buf = self.item_output_buf.clone();
        for val in item_output_buf.values_mut() {
            *val = 0;
        }

        Box::from(Recycler {
            name: self.name.clone(),
            texture: self.texture,

            period: self.period,
            from_last_production: 0,
            can_produce: false,

            item_input: self.item_input.clone(),
            item_output: self.item_output.clone(),

            item_input_buf,
            item_output_buf,

            item_prototypes: self.item_prototypes.clone(),
        })
    }
}

impl Building for Recycler {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl MessageSender for Recycler {
    fn pull_messages(&mut self, tick_id: u32) -> Vec<Message> {
        let msgs = self.pull_item_messages(tick_id);
        msgs
    }
}

impl MessageReceiver for Recycler {
    fn try_push_message(&mut self, mut message: Message) -> Option<Message> {
        // match &message.body {
        //     MessageBody::PushItem(item) => {
        //         let item_id = item.get_id();
        //         if self.item_input.contains_key(&item_id) {
        //             let inp_buf = self.item_input_buf.get_mut(&item_id).unwrap();
        //             if *inp_buf < *self.item_input.get(&item_id).unwrap() {
        //                 *inp_buf += 1;
        //                 return None;
        //             }
        //         }
        //         Some(message)
        //     }
        //     _ => Some(message),
        // }

        match &message {
            Message::FieldMessage(msg) => match &msg.body {
                field_message::MessageBody::PushItem(item) => {
                    if msg.refund {
                        *self.item_output_buf.get_mut(&item.get_id()).unwrap() += 1;
                        None
                    } else {
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
                }
                _ => Some(message),
            },
            _ => Some(message),
        }
    }
}
