use crate::game::field::message::*;
use crate::game::game_entity::*;

use crate::game::physics_scene::message as physics_message;
use crate::game::physics_scene::{BodyCollection, PhysicsSimulated};

#[derive(Default)]
pub struct Cell {}

impl GameEntity for Cell {
    fn update(&mut self, parameters: &UpdateParameters) {}

    fn tick(&mut self, tick_id: u32) {}

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {}
}

impl MessageReceiver for Cell {
    fn try_push_message(&mut self, message: Message) -> Option<Message> {
        Some(message)
    }
}

impl MessageSender for Cell {
    fn pull_messages(&mut self, tick_id: u32) -> Vec<Message> {
        vec![]
    }

    fn message_send_result(&mut self, result: MessageSendResult) {}
}

impl PhysicsSimulated for Cell {
    fn get_all_bodies(&mut self) -> BodyCollection {
        BodyCollection::new()
    }

    fn handle_physics_messages(&mut self, messages: Vec<physics_message::Message>) {}
}
