use crate::common::math::IVec2;
use crate::game::building::transport_belt::TransportedItem;

pub struct Message {
    pub sender : IVec2,
    pub receiver : IVec2,
    pub tick_id : u32,
    pub body : MessageBody
}   

pub enum MessageBody {
    PushItem(TransportedItem)
}

pub trait MessageReceiver {
    // Returns message if not processed.
    fn try_push_message(&mut self, message : Message) -> Option<Message>;
}

pub trait MessageSender {
    fn pull_messages(&mut self) -> Vec<Message>;
    fn push_back_message(&mut self, message : Message);
}