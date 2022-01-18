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

pub struct MessageSendResult {
    pub message_id : u32,
    // If message is Some it means that message is received by receiver
    // elsewhere it failed to send.
    pub message : Option<Message>,
    pub tick_id : u32
}

pub trait MessageSender {
    fn pull_messages(&mut self, tick_id : u32) -> Vec<Message>;
    fn message_send_result(&mut self, result : MessageSendResult);
}