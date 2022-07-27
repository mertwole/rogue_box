use crate::game::{
    common::{direction::Direction, math::IVec2},
    location::field::building::item::TransportedItem,
};

pub struct Message {
    // Id local for sender.
    pub id: u32,
    pub sender: MessageExchangeActor,
    pub receiver: MessageExchangeActor,
    pub target: Target,
    pub tick_id: u32,
    pub refund: bool,
    pub body: MessageBody,
}

pub enum Target {
    Directions(Vec<Direction>),
}

#[derive(Clone, Default)]
pub struct MessageExchangeActor {
    pub position: Option<IVec2>,
}

impl MessageExchangeActor {
    pub fn get_position(&self) -> IVec2 {
        self.position.unwrap_or_else(|| {
            log::error!("Reading MessageExchangeActor when set to None");
            IVec2::zero()
        })
    }

    pub fn set_position(&mut self, position: IVec2) {
        self.position = Some(position);
    }
}

pub enum MessageBody {
    PushItem(TransportedItem),
}
