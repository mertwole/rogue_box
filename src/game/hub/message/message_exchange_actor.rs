use crate::game::common::math::IVec2;
use crate::game::hub::electric_port::PortId;

#[derive(Clone)]
pub struct MessageExchangeActor {
    pub position : Option<IVec2>,
    pub electric_port : Option<PortId>
}

impl MessageExchangeActor {
    pub fn new() -> MessageExchangeActor {
        MessageExchangeActor {
            position : None,
            electric_port : None
        }
    }

    pub fn at_position(position : IVec2) -> MessageExchangeActor {
        MessageExchangeActor {
            position : Some(position),
            electric_port : None
        }
    }

    pub fn get_position(&self) -> IVec2 {
        self.position.unwrap_or_else(|| {
            log::warn!("Reading position of MessageExchangeActor when it's None");  
            IVec2::zero() 
        })
    }

    pub fn set_position(&mut self, position : IVec2) {
        self.position = Some(position)
    }

    pub fn get_electric_port(&self) -> PortId {
        self.electric_port.unwrap_or_else(|| {
            log::warn!("Reading electric port of MessageExchangeActor when it's None");  
            PortId::new(0)
        })
    }

    pub fn set_electric_port(&mut self, id : PortId) {
        self.electric_port = Some(id)
    }
}