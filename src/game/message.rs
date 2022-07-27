pub use crate::game::location::{
    field::message as field_message, physics_scene::message as physics_message,
};

pub enum Message {
    FieldMessage(field_message::Message),
}

pub trait MessageSender {
    fn pull_messages(&mut self, tick_id: u32) -> Vec<Message>;
}

pub trait MessageReceiver {
    // Returns message if not processed.
    fn try_push_message(&mut self, message: Message) -> Option<Message>;
}
