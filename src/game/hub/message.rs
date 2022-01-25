use crate::game::common::math::IVec2;
use crate::game::hub::item::TransportedItem;
use crate::game::common::direction::Direction;
use crate::game::hub::electric_port::{WattTick, PortId};

pub struct Message {
    // Id local for sender.
    pub id : u32,
    pub sender : MessageExchangeActor,
    pub receiver : MessageExchangeActor,
    pub target : Target,
    pub tick_id : u32,
    pub body : MessageBody
}

pub enum Target {
    Direction(Direction),
    BroadcastNeighbors,
    BroadcastAllConnectedElectricInputs
}

pub enum MessageExchangeActor {
    AtPosition(IVec2),
    NotComputedYet
}

impl MessageExchangeActor {
    pub fn get_pos(&self) -> IVec2 {
        match self {
            MessageExchangeActor::AtPosition(pos) => { *pos }
            MessageExchangeActor::NotComputedYet => { 
                log::warn!("Trying to get MessageExchangeActor position when it's not computed.");
                IVec2::zero() 
            }
        }
    }
}

pub enum MessageBody {
    PushItem(TransportedItem),
    SendElectricity(WattTick, PortId)
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