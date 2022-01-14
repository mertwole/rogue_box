use super::{Building, BuildingClone};
use crate::game::game_entity::*;
use crate::common::direction::Direction;
use crate::common::json_reader::JsonReader;
use crate::common::math::IVec2;

pub mod transport_belt_manager;

pub struct TransportBelt { 
    name : String,

    pub /*DEBUG*/ inputs : Vec<Direction>,
    pub /*DEBUG*/ output : Direction,
    // Item count on the one side of the belt
    // so max capacity of belt = item_count * 4 + 1(center).
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
            item_count,
            position : IVec2::zero()
        }    
    }

    pub fn check_can_connect(&self, other : &TransportBelt) -> bool {
        if self.output.to_ivec2() + self.position == other.position { return true; }
        if other.output.to_ivec2() + other.position == self.position { return true; }
        false
    }

    fn alt_tick(&mut self) {
        
    }   
}

impl GameEntity for TransportBelt {
    fn update(&mut self, delta_time : f32) {

    }

    fn tick(&mut self) {
        // Do nothing because TransportBelt tick is driven by TransportBeltManager
        // and occurs in alt_tick().
    }

    fn render(&mut self, renderer : &mut Renderer) {

    }
}

impl BuildingClone for TransportBelt {
    fn clone_box(&self) -> Box<dyn Building> {
        Box::from(TransportBelt { 
            name : self.name.clone(),
            inputs : Vec::new(),
            output : Direction::None,
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